use super::*;

#[derive(Debug)]
pub struct Builder<'a> {
	unit: Unit<'a, InConstruction>,
	is_terminated: bool,
}

pub fn ret(value: &[InstIdx]) -> (BlockIdx, &[InstIdx]) {
	((u16::MAX as usize).into(), value)
}

impl<'a> Builder<'a> {
	pub fn new(signature: &'a [Type]) -> Self {
    	Self {
			unit: Unit::new(signature),
			is_terminated: false,
    	}
	}
	pub fn finalize(self, return_sig: &'a [Type]) -> Unit<Finalized> {
    	assert!(self.is_terminated);
		self.unit.finalize(return_sig)
	}
    pub fn push<'b>(&'b mut self) -> InstStack<'b, 'a> {InstStack(self)}
	pub fn new_block(&mut self, sig: &[Type]) -> BlockIdx {
        assert!(self.is_terminated);
        let sig = self.unit.sigs.append(sig);
        self.is_terminated = false;
        self.unit.blocks.push(Block {sig, start: self.unit.insts.new_key(), end: 0.into()})
	}
}

#[derive(Debug)]
pub struct InstStack<'a, 'b: 'a>(&'a mut Builder<'b>);
impl<'a, 'b: 'a> InstStack<'a, 'b> {
    fn terminate(&mut self) {
        assert!(!self.0.is_terminated);
        self.0.is_terminated = true;
        self.0.unit.blocks.last_mut().unwrap().end = self.0.unit.insts.new_key();
    }
    fn expr(&mut self) {
		assert!(!self.0.is_terminated);
    }
    pub fn fetch_arg(&mut self, n: u32) -> InstIdx {
        self.expr();
        self.0.unit.insts.push(Instruction::FetchArg(n))
    }
    pub fn iconst(&mut self, int: u64) -> InstIdx {
        self.expr();
        self.0.unit.insts.push(Instruction::IntConst(int.to_ne_bytes()))
    }
    pub fn add(&mut self, v1: InstIdx, v2: InstIdx) -> InstIdx {
        self.expr();
		self.0.unit.insts.push(Instruction::Add(v1, v2))
    }
    pub fn sub(&mut self, v1: InstIdx, v2: InstIdx) -> InstIdx {
        self.expr();
		self.0.unit.insts.push(Instruction::Sub(v1, v2))
    }
    pub fn mult(&mut self, v1: InstIdx, v2: InstIdx) -> InstIdx {
        self.expr();
		self.0.unit.insts.push(Instruction::Mult(v1, v2))
    }
    pub fn div(&mut self, v1: InstIdx, v2: InstIdx) -> InstIdx {
        self.expr();
		self.0.unit.insts.push(Instruction::Div(v1, v2))
    }
    pub fn less(&mut self, v1: InstIdx, v2: InstIdx) -> InstIdx {
        self.expr();
		self.0.unit.insts.push(Instruction::Less(v1, v2))
    }
    pub fn more(&mut self, v1: InstIdx, v2: InstIdx) -> InstIdx {
        self.expr();
		self.0.unit.insts.push(Instruction::More(v1, v2))
    }
    pub fn equal(&mut self, v1: InstIdx, v2: InstIdx) -> InstIdx {
        self.expr();
		self.0.unit.insts.push(Instruction::Equal(v1, v2))
    }
    pub fn call(&mut self, func: FuncRef, args: &[InstIdx]) -> InstIdx {
		self.expr();
		let args = self.0.unit.extra_args.append_from_iter(args.iter().map(|a| a.0));
		self.0.unit.insts.push(Instruction::Call(func, args))
    }
    pub fn recurse(&mut self, args: &[InstIdx]) -> InstIdx {
		self.expr();
		let args = self.0.unit.extra_args.append_from_iter(args.iter().map(|a| a.0));
		self.0.unit.insts.push(Instruction::Call(FuncRef::from(0), args))
    }
    pub fn branch(&mut self, branch: (BlockIdx, &[InstIdx])) {
		self.terminate();
  		let a = self.0.unit.extra_args.append_from_iter(branch.1.iter().map(|i| i.0));
		self.0.unit.insts.push(Instruction::Branch(branch.0, a));
    }
    pub fn branch_if(&mut self, value: InstIdx, branches: [(BlockIdx, &[InstIdx]);2]) {
		self.0.unit.insts.push(Instruction::DoIf(value));
  		let a = self.0.unit.extra_args.append_from_iter(branches[0].1.iter().map(|i| i.0));
		self.0.unit.insts.push(Instruction::Branch(branches[0].0, a));
        self.terminate();
  		let a = self.0.unit.extra_args.append_from_iter(branches[1].1.iter().map(|i| i.0));
		self.0.unit.insts.push(Instruction::Branch(branches[1].0, a));
    }
}

