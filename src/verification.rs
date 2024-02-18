use crate::data::{Block, BlockData, InstData, InstKind, Instruction, Set, Type, Unit, LiveData, DataPart};
use crate::util::KeyVec;

impl Unit {
    pub(crate) fn annotate_liveness(&mut self) {
        let mut visited = Vec::new();
        // do a depth-first-traversal of the cfg
        // and determine liveness of each used instruction
        self.set_liveness(Block(0), &mut visited);
    }
    fn set_liveness(&mut self, block: Block, visited: &mut Vec<Block>) -> Set<Instruction> {
        let scope = visited.len();
        visited.push(block);
        let mut set = Set::new();
        let blockdata = &self.blocks[block];
        let first = blockdata.inst_range[0].0;
        let last = blockdata.inst_range[1].0;
        for c in blockdata.get_next(self).into_iter().filter_map(|t| t) {
            // determine values that pass the block boundary
            // and mark them as Alive
            let mut s = self.set_liveness(c, visited);
            for i in s.iter() {
                self.liveness.insert((block, *i), LiveData::Alive);
            }
            set.append(&mut s);
        }
        // determine partial liveness
        for i in (first..last).rev().map(|i| Instruction(i)) {
            set.remove(&i);
            let used = self.instructions[i].kind.get_insts(&self.data);
            for u in used.into_iter() {
                if !self.liveness.contains_key(&(block, *u)) {
                    self.liveness.insert((block, *u), LiveData::Partial(i));
                    set.insert(*u);
                }
            }
        }
        visited.truncate(scope);
        set
    }
    fn get_dependencies(&mut self, block: Block, visited: &mut Vec<Block>) -> Set<Block> {
        let scope = visited.len();
        visited.push(block);
        let mut set = Set::new();
        let blockdata = &self.blocks[block];
        let first = blockdata.inst_range[0].0;
        let last = blockdata.inst_range[1].0;
        // determine direct dependency
        // (use of instructions directly by this block,
        //  including self)
        for i in (first..last).map(|i| Instruction(i)) {
            let inst = &self.instructions[i];
            set.insert(inst.block);
        }
        // determine indirect dependency
        for c in blockdata.get_next(self).into_iter().filter_map(|t| t) {
            if !visited.contains(&c) {
                set.append(&mut self.get_dependencies(c, visited));
            }
        }
        // satisfy dependency on self
        set.remove(&block);
        visited.truncate(scope);
        set
    }
    pub(crate) fn check_dependencies(&mut self) {
        let mut visited = Vec::new();
        // if any dependency bubbled up to block 0, then
        // there must be a codepath that does not fulfill
        // the dependency, aka a reqd. block is not a dominator
        let set = self.get_dependencies(Block(0), &mut visited);
        if !set.is_empty() {
            panic!("aaaa");
        }
    }
    fn width_first_traversal<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut Self, Block),
    {
        let mut order = vec![Block(0)];
        let mut i = 0;
        while let Some(block) = order.get(i).cloned() {
            let [a, b] = self.blocks[block].get_next(self);
            a.map(|a| {
                if !order.contains(&a) {
                    order.push(a)
                }
            });
            b.map(|b| {
                if !order.contains(&b) {
                    order.push(b)
                }
            });
            callback(self, block);
            i += 1;
        }
    }
    pub(crate) fn remove_unused(&mut self) {
        let mut unused = vec![true; self.instructions.len()];
        self.width_first_traversal(|unit, block| {
            let blockdata = &unit.blocks[block];
            let first = blockdata.inst_range[0].0;
            let last = blockdata.inst_range[1].0;
            for inst in (first..last).rev() {
                let instruction = &unit.instructions[Instruction(inst)];
                if instruction.kind.is_term() {
                    unused[inst as usize] = false;
                }
                if unused[inst as usize] {
                    continue;
                }
                let refs = instruction.kind.get_insts(&unit.data);
                for i in refs {
                    unused[i.0 as usize] = false;
                }
            }
        });
        for i in unused
            .into_iter()
            .enumerate()
            .filter(|(_, b)| *b)
            .map(|(i, _)| Instruction(i as u32))
        {
            if self.settings.volatile {
                panic!("Instruction {} is unused but volatile is set!", i);
            }
            let inst = InstData {
                block: self.instructions[i].block,
                kind: InstKind::Tombstone,
                typing: Type::Void,
            };
            self.instructions[i] = inst;
        }
    }
}

impl BlockData {
    // returns None for the return block index as it does not count as a block
    fn get_next(&self, unit: &Unit) -> [Option<Block>; 2] {
        let [inst_start, inst_end] = self.inst_range;
        let mut out = [
            unit.instructions
                .get(inst_end)
                .map(|i| i.kind.get_block())
                .flatten(),
            unit.instructions
                .get(Instruction(inst_end.0 - 1))
                .map(|i| i.kind.get_block())
                .flatten(),
        ];
        if inst_end == inst_start {
            out[1] = None;
        }
        out
    }
}

impl InstKind {
    fn get_block(&self) -> Option<Block> {
        match self {
            Self::Terminator(crate::data::TermData::Branch(Block::MAX, _)) => None,
            Self::Terminator(crate::data::TermData::Branch(b, _)) => Some(*b),
            _ => None,
        }
    }
    fn get_insts<'a>(&'a self, data: &'a KeyVec<DataPart, Instruction>) -> &'a [Instruction] {
        match self {
            Self::Add(a) | Self::Sub(a) | Self::Less(a) | Self::More(a) => a,
            Self::Recur(a) | Self::Terminator(crate::data::TermData::Branch(_, a)) => {
                &data[*a]
            }
            Self::Terminator(crate::data::TermData::DoIf(i)) => std::slice::from_ref(i),
            _ => &[],
        }
    }
    fn is_term(&self) -> bool {
        match self {
            Self::Terminator(_) => true,
            _ => false,
        }
    }
}
