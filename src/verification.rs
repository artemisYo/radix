use crate::data::{BlockData, Block, InstKind, InstData, Instruction, Unit, Type, Set};

impl Unit {
	fn get_dependencies(&self, block: Block, visited: &mut Vec<Block>) -> Set<Block> {
		let scope = visited.len();
		visited.push(block);
		let blockdata = &self.blocks[block];
    	let mut out = Set::from_iter(blockdata.dd.iter().cloned());
		for c in blockdata.get_next(self)
    		.into_iter()
    		.filter_map(|t| t)
    	{
        	if !visited.contains(&c) {
            	out.append(&mut self.get_dependencies(c, visited));
        	}
		}
		out.remove(&block);
		visited.truncate(scope);
		out
	}
    pub(crate) fn check_dependencies(&mut self) {
        let mut visited = Vec::new();
        if !self.get_dependencies(Block(0), &mut visited).is_empty() {
			panic!("aaaa");
        }
    }
    fn width_first_traversal<F>(&mut self, mut callback: F)
    where F: FnMut(&mut Self, Block)
    {
		let mut order = vec![Block(0)];
		let mut i = 0;
		while let Some(block) = order.get(i).cloned() {
			let [a, b] = self.blocks[block].get_next(self);
			a.map(|a| if !order.contains(&a) { order.push(a) });
			b.map(|b| if !order.contains(&b) { order.push(b) });
			callback(self, block);
			i += 1;
		}
    }
    pub(crate) fn remove_unused(&mut self) {
		let mut unused = vec![true; self.instructions.len()];
		self.width_first_traversal(|unit, block| {
			let blockdata = &unit.blocks[block];
			let last = blockdata.end.0;
			let first = blockdata.start.0;
			for inst in (first..=last).rev() {
				let instruction = &unit.instructions[Instruction(inst)];
				if instruction.kind.is_term() {
					unused[inst as usize] = false;
				}
    			if unused[inst as usize] {
					continue;
    			}
				let refs = instruction.kind.get_insts(unit);
				for i in refs {
					unused[i.0 as usize] = false;
				}
			}
		});
		for i in unused.into_iter()
    		.enumerate()
    		.filter(|(_, b)| *b)
    		.map(|(i, _)| Instruction(i as u32)) {
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
        let mut out = [
        	unit.instructions.get(self.end).map(|i| i.kind.get_block()).flatten(),
        	unit.instructions.get(Instruction(self.end.0 - 1)).map(|i| i.kind.get_block()).flatten(),
        ];
        if self.end == self.start {
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
    fn get_insts<'a>(&'a self, unit: &'a Unit) -> &'a [Instruction] {
        match self {
            Self::Add(a) | Self::Sub(a) | Self::Less(a) | Self::More(a) => a,
            Self::Recur(a)
            | Self::Terminator(crate::data::TermData::Branch(_, a)) => &unit.data[*a],
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
