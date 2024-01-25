use crate::{Unit, data::InstData, data::Instruction};

impl Unit {
	pub(crate) fn remove_unused(&mut self) {
    	let mut unused = vec![true; self.instructions.len()];
    	for inst in self.instructions.iter() {
        	for i in inst.get_insts().into_iter().filter(Option::is_some) {
				unused[i.unwrap().0 as usize] = false;
        	}
    	}
    	for idx in unused.into_iter()
        	.enumerate()
        	.filter(|(_, b)| *b)
        	.map(|(i, _)| Instruction(i as u32)) {
			self.instructions[idx] = InstData::Tombstone;
        }
	}
}

impl InstData {
	fn get_insts(&self) -> [Option<Instruction>;2] {
		match self {
			Self::Add([a, b])
    		| Self::Sub([a, b])
    		| Self::Less([a, b])
    		| Self::More([a, b]) => [Some(*a), Some(*b)],
    		Self::Terminator(crate::data::TermData::DoIf(i)) => [Some(*i), None],
    		_ => [None, None],
		}
	}
}
