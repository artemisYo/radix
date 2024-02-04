use crate::builder::Builder;
use crate::data::{InstData, Instruction, Type};

impl<'a, Seal> Builder<'a, Seal> {
    pub fn fetch_arg(&mut self, index: usize) -> Instruction {
        let s = self.handle.blocks[self.block.index].signature;
        let t = self.handle.signatures[s][index];
        self.handle.instructions.push((t, InstData::FetchArg(index)))
    }
    pub fn iconst(&mut self, t: Type, number: isize) -> Instruction {
        self.handle.instructions.push((t, InstData::IConst(number)))
    }
    pub fn less(&mut self, a: Instruction, b: Instruction) -> Instruction {
        let t = self.handle.instructions[a].0;
        self.handle.instructions.push((t, InstData::Less([a, b])))
    }
    pub fn more(&mut self, a: Instruction, b: Instruction) -> Instruction {
        let t = self.handle.instructions[a].0;
        self.handle.instructions.push((t, InstData::More([a, b])))
    }
    pub fn add(&mut self, a: Instruction, b: Instruction) -> Instruction {
        let t = self.handle.instructions[a].0;
        self.handle.instructions.push((t, InstData::Add([a, b])))
    }
    pub fn sub(&mut self, a: Instruction, b: Instruction) -> Instruction {
        let t = self.handle.instructions[a].0;
        self.handle.instructions.push((t, InstData::Sub([a, b])))
    }
    pub fn recurse(&mut self, args: &[Instruction]) -> Instruction {
        let data = self.handle.data.push_slice(args);
        let t = self.handle.retsig.unwrap_or(Type::Void);
        self.handle.instructions.push((t, InstData::Recur(data)))
    }
}
