use crate::data::BlockData;
use crate::{data::Block, data::InstData, data::Instruction, Unit};

impl Unit {
    fn width_first_ordering(&self) -> Vec<Block> {
		let mut order = vec![Block(0)];
		let mut i = 0;
		while let Some(block) = order.get(i).cloned() {
			let [a, b] = self.blocks[block].get_next(self);
			a.map(|a| if !order.contains(&a) { order.push(a) });
			b.map(|b| if !order.contains(&b) { order.push(b) });
			i += 1;
		}
		order
    }
    pub(crate) fn remove_unused(&mut self) {
		let order = self.width_first_ordering();
		let mut unused = vec![true; self.instructions.len()];
		for block in order.into_iter().rev() {
    		let blockdata = &self.blocks[block];
    		let last = blockdata.end.0;
    		let first = blockdata.start.0;
			for inst in (first..=last).rev() {
				let instruction = &self.instructions[Instruction(inst)];
				if instruction.is_term() {
					unused[inst as usize] = false;
				}
    			if unused[inst as usize] {
					continue;
    			}
				let refs = instruction.get_insts(self);
				for i in refs {
					unused[i.0 as usize] = false;
				}
			}
		}
		for i in unused.into_iter()
    		.enumerate()
    		.filter(|(_, b)| *b)
    		.map(|(i, _)| Instruction(i as u32)) {
			self.instructions[i] = InstData::Tombstone;
    	}
    }
}

impl BlockData {
    // returns None for the return block index as it does not count as a block
    fn get_next(&self, unit: &Unit) -> [Option<Block>; 2] {
        let mut out = [
        	unit.instructions.get(self.end).map(InstData::get_block).flatten(),
        	unit.instructions.get(Instruction(self.end.0 - 1)).map(InstData::get_block).flatten(),
        ];
        if self.end == self.start {
            out[1] = None;
        }
        out
    }
}

impl InstData {
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
