use crate::builder::Builder;
use crate::data::{InstKind, InstData, Instruction, Type};

impl<'a> Builder<'a> {
    pub fn fetch_arg(&mut self, index: usize) -> Instruction {
        let s = self.handle.blocks[self.block.index].signature;
        let t = self.handle.signatures[s][index];
        let inst = InstData {
            block: self.block.index,
            kind: InstKind::FetchArg(index),
            typing: t,
        };
        self.handle.instructions.push(inst)
    }
    pub fn iconst(&mut self, t: Type, number: isize) -> Instruction {
        let inst = InstData {
            block: self.block.index,
            kind: InstKind::IConst(number),
            typing: t,
        };
        self.handle.instructions.push(inst)
    }
    pub fn less(&mut self, a: Instruction, b: Instruction) -> Instruction {
        self.register_dd([a, b]);
        let t = self.handle.instructions[a].typing;
        let inst = InstData {
            block: self.block.index,
            kind: InstKind::Less([a, b]),
            typing: t,
        };
        self.handle.instructions.push(inst)
    }
    pub fn more(&mut self, a: Instruction, b: Instruction) -> Instruction {
        self.register_dd([a, b]);
        let t = self.handle.instructions[a].typing;
        let inst = InstData {
            block: self.block.index,
            kind: InstKind::More([a, b]),
            typing: t,
        };
        self.handle.instructions.push(inst)
    }
    pub fn add(&mut self, a: Instruction, b: Instruction) -> Instruction {
        self.register_dd([a, b]);
        let t = self.handle.instructions[a].typing;
        let inst = InstData {
            block: self.block.index,
            kind: InstKind::Add([a, b]),
            typing: t,
        };
        self.handle.instructions.push(inst)
    }
    pub fn sub(&mut self, a: Instruction, b: Instruction) -> Instruction {
        self.register_dd([a, b]);
        let t = self.handle.instructions[a].typing;
        let inst = InstData {
            block: self.block.index,
            kind: InstKind::Sub([a, b]),
            typing: t,
        };
        self.handle.instructions.push(inst)
    }
    pub fn recurse(&mut self, args: &[Instruction]) -> Instruction {
        self.register_dd(args.into_iter().cloned());
        let data = self.handle.data.push_slice(args);
        let t = self.handle.retsig.unwrap_or(Type::Void);
        let inst = InstData {
            block: self.block.index,
            kind: InstKind::Recur(data),
            typing: t,
        };
        self.handle.instructions.push(inst)
    }
}
