use super::{*, blocks::{InstBuilder, TerminatorBuilder}};

#[derive(Debug)]
pub enum Instruction {
    // mostly used for deleting instructions
    Nop,
    FetchArg(u32),
    IntConst([u8;8]),
    Add(InstIdx, InstIdx),
    Sub(InstIdx, InstIdx),
    Mult(InstIdx, InstIdx),
    Div(InstIdx, InstIdx),
    Less(InstIdx, InstIdx),
    More(InstIdx, InstIdx),
    Equal(InstIdx, InstIdx),
    Call(FuncRef, ArgIdx),
    // used for ints > 32 bits
    DoIf(InstIdx),
    Branch(BlockIdx, ArgIdx),
}
impl Instruction {
    fn is_term(&self) -> bool {
        use Instruction::*;
        match self {
            Branch(_, _) => true,
            _ => false
        }
    }
}

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

pub fn ret(value: &[InstIdx]) -> (BlockIdx, &[InstIdx]) {
	((u16::MAX as usize).into(), value)
}

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
