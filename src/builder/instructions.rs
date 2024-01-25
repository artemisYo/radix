use crate::builder::Builder;
use crate::data::{InstData, Instruction};

impl<'a, Seal> Builder<'a, Seal> {
    pub fn fetch_arg(&mut self, index: usize) -> Instruction {
        self.handle.instructions.push(InstData::FetchArg(index))
    }
    pub fn iconst(&mut self, number: isize) -> Instruction {
        self.handle.instructions.push(InstData::IConst(number))
    }
    pub fn less(&mut self, a: Instruction, b: Instruction) -> Instruction {
        self.handle.instructions.push(InstData::Less([a, b]))
    }
    pub fn more(&mut self, a: Instruction, b: Instruction) -> Instruction {
        self.handle.instructions.push(InstData::More([a, b]))
    }
    pub fn add(&mut self, a: Instruction, b: Instruction) -> Instruction {
        self.handle.instructions.push(InstData::Add([a, b]))
    }
    pub fn sub(&mut self, a: Instruction, b: Instruction) -> Instruction {
        self.handle.instructions.push(InstData::Sub([a, b]))
    }
    pub fn recurse(&mut self, args: &[Instruction]) -> Instruction {
        let data = self.handle.data.push_slice(args);
        self.handle.instructions.push(InstData::Recur(data))
    }
}
