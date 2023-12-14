use super::*;

pub trait BuilderHandleable {}
impl BuilderHandleable for () {}
impl BuilderHandleable for &mut Unit<'_, InConstruction> {}

pub struct InstBuilder<'a, 'b: 'a, 'c: 'b>(&'a mut BlockHandle<&'b mut Unit<'c, InConstruction>>);
impl<'a> InstBuilder<'a, '_, '_> {
    pub fn fetch_arg(&mut self, n: u32) -> InstIdx {
        self.0.handle.insts.push(Instruction::FetchArg(n))
    }
    pub fn iconst(&mut self, int: u64) -> InstIdx {
        self.0.handle.insts.push(Instruction::IntConst(int.to_ne_bytes()))
    }
    pub fn add(&mut self, v1: InstIdx, v2: InstIdx) -> InstIdx {
		self.0.handle.insts.push(Instruction::Add(v1, v2))
    }
    pub fn sub(&mut self, v1: InstIdx, v2: InstIdx) -> InstIdx {
		self.0.handle.insts.push(Instruction::Sub(v1, v2))
    }
    pub fn mult(&mut self, v1: InstIdx, v2: InstIdx) -> InstIdx {
		self.0.handle.insts.push(Instruction::Mult(v1, v2))
    }
    pub fn div(&mut self, v1: InstIdx, v2: InstIdx) -> InstIdx {
		self.0.handle.insts.push(Instruction::Div(v1, v2))
    }
    pub fn less(&mut self, v1: InstIdx, v2: InstIdx) -> InstIdx {
		self.0.handle.insts.push(Instruction::Less(v1, v2))
    }
    pub fn more(&mut self, v1: InstIdx, v2: InstIdx) -> InstIdx {
		self.0.handle.insts.push(Instruction::More(v1, v2))
    }
    pub fn equal(&mut self, v1: InstIdx, v2: InstIdx) -> InstIdx {
		self.0.handle.insts.push(Instruction::Equal(v1, v2))
    }
    pub fn call(&mut self, func: FuncRef, args: &[InstIdx]) -> InstIdx {
		let args = self.0.handle.extra_args.append_from_iter(args.iter().map(|a| a.0));
		self.0.handle.insts.push(Instruction::Call(func, args))
    }
    pub fn recurse(&mut self, args: &[InstIdx]) -> InstIdx {
		let args = self.0.handle.extra_args.append_from_iter(args.iter().map(|a| a.0));
		self.0.handle.insts.push(Instruction::Call(0.into(), args))
    }
}

pub struct TerminatorBuilder<'a, 'b>(BlockHandle<&'a mut Unit<'b, InConstruction>>);
impl TerminatorBuilder<'_, '_> {
    pub fn branch<T>(&mut self, branch: (T, &[InstIdx]))
        where T: Into<BlockIdx> {
  		let a = self.0.handle.extra_args.append_from_iter(branch.1.iter().map(|i| i.0));
		self.0.handle.insts.push(Instruction::Branch(branch.0.into(), a));
    }
    pub fn branch_if<T>(&mut self, value: InstIdx, branches: [(T, &[InstIdx]);2])
        where T: Into<BlockIdx> {
        let [br1, br2] = branches;
		self.0.handle.insts.push(Instruction::DoIf(value));
  		let a = self.0.handle.extra_args.append_from_iter(br1.1.iter().map(|i| i.0));
		self.0.handle.insts.push(Instruction::Branch(br1.0.into(), a));
  		let a = self.0.handle.extra_args.append_from_iter(br2.1.iter().map(|i| i.0));
		self.0.handle.insts.push(Instruction::Branch(br2.0.into(), a));
    }
}
impl Drop for TerminatorBuilder<'_, '_> {
    fn drop(&mut self) {
        self.0.handle.blocks[self.0.index].end = self.0.handle.insts.last_key();
    }
}

pub struct BlockHandle<T: BuilderHandleable> {
	index: BlockIdx,
	handle: T
}
impl<T: BuilderHandleable> From<&BlockHandle<T>> for BlockIdx {
    fn from(value: &BlockHandle<T>) -> Self {
        value.index
    }
}

impl<'a, 'b: 'a> BlockHandle<&'a mut Unit<'b, InConstruction>> {
    pub fn push<'c>(&'c mut self) -> InstBuilder<'c, 'a, 'b> {
        let block = &mut self.handle.blocks[self.index];
        assert!(block.end.0 == 0);
        if block.start.0 == 0 {
			block.start = self.handle.insts.new_key();
        }
		InstBuilder(self)
    }
    pub fn terminate(self) -> TerminatorBuilder<'a, 'b> {
		TerminatorBuilder(self)
    }
}

impl Unit<'_, InConstruction> {
    pub fn entry_block(&self) -> BlockHandle<()> {
		BlockHandle {index: 0.into(), handle: ()}
    }
	pub fn new_block(&mut self, sig: &[Type]) -> BlockHandle<()> {
        let sig = self.sigs.append(sig);
        let index = self.blocks.push(Block {sig, start: 0.into(), end: 0.into()});
		BlockHandle {index, handle: ()}
	}
	pub fn switch_to(&mut self, block: &BlockHandle<()>) -> BlockHandle<&mut Self> {
		BlockHandle {
			index: block.index,
			handle: self
		}
	}
}
