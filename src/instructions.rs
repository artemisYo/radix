use crate::data::{InstData, Instruction};
use crate::builder::BlockBuilder;

impl<'a, Seal> BlockBuilder<'a, Seal> {
    pub fn fetch_arg(&mut self, index: usize) -> Instruction {
        self.0.handle.instructions.push(
            InstData::FetchArg(index)
        )        
    }
    pub fn iconst(&mut self, number: isize) -> Instruction {
        self.0.handle.instructions.push(
            InstData::IConst(number)
        )
    }
    pub fn less(&mut self, a: Instruction, b: Instruction) -> Instruction {
        self.0.handle.instructions.push(
            InstData::Less([a, b])
        )
    }
    pub fn more(&mut self, a: Instruction, b: Instruction) -> Instruction {
        self.0.handle.instructions.push(
            InstData::More([a, b])
        )
    }
    pub fn add(&mut self, a: Instruction, b: Instruction) -> Instruction {
        self.0.handle.instructions.push(
            InstData::Add([a, b])
        )
    }
    pub fn sub(&mut self, a: Instruction, b: Instruction) -> Instruction {
        self.0.handle.instructions.push(
            InstData::Sub([a, b])
        )
    }
    pub fn recurse(&mut self, args: &[Instruction]) -> Instruction {
        let data = self.0.handle.data.push_iter(args.into_iter().map(|a| a.0));
        let _ = data;
        self.0.handle.instructions.push(
            InstData::Recur(data)
        )
    }
}
