use crate::builder::Builder;
use crate::data::{InstData, InstKind, Instruction, Type, UseMetadata};

impl<'a> Builder<'a> {
    pub(crate) fn push_inst(&mut self, inst: InstData, used: &[Instruction]) -> Instruction {
        for used in used.iter().cloned() {
            self.handle.use_meta.push(UseMetadata {
                location: self.handle.instructions.next_idx(),
                is_final: false,
                used,
            });
        }
        self.handle.instructions.push(inst)
    }
    pub fn fetch_arg(&mut self, index: usize) -> Instruction {
        let s = self.handle.blocks[self.block.index].signature;
        let t = self.handle.signatures[s][index];
        let inst = InstData {
            block: self.block.index,
            kind: InstKind::FetchArg(index),
            typing: t,
        };
        self.push_inst(inst, &[])
    }
    pub fn iconst(&mut self, t: Type, number: isize) -> Instruction {
        let inst = InstData {
            block: self.block.index,
            kind: InstKind::IConst(number),
            typing: t,
        };
        self.push_inst(inst, &[])
    }
    pub fn less(&mut self, args: [Instruction; 2]) -> Instruction {
        self.register_dd(&args);
        let t = self.handle.instructions[args[0]].typing;
        let inst = InstData {
            block: self.block.index,
            kind: InstKind::Less(args),
            typing: t,
        };
        self.push_inst(inst, &args)
    }
    pub fn more(&mut self, args: [Instruction; 2]) -> Instruction {
        self.register_dd(&args);
        let t = self.handle.instructions[args[0]].typing;
        let inst = InstData {
            block: self.block.index,
            kind: InstKind::More(args),
            typing: t,
        };
        self.push_inst(inst, &args)
    }
    pub fn add(&mut self, args: [Instruction; 2]) -> Instruction {
        self.register_dd(&args);
        let t = self.handle.instructions[args[0]].typing;
        let inst = InstData {
            block: self.block.index,
            kind: InstKind::Add(args),
            typing: t,
        };
        self.push_inst(inst, &args)
    }
    pub fn sub(&mut self, args: [Instruction; 2]) -> Instruction {
        self.register_dd(&args);
        let t = self.handle.instructions[args[0]].typing;
        let inst = InstData {
            block: self.block.index,
            kind: InstKind::Sub(args),
            typing: t,
        };
        self.push_inst(inst, &args)
    }
    pub fn recurse(&mut self, args: &[Instruction]) -> Instruction {
        self.register_dd(args);
        let data = self.handle.data.push_slice(args);
        let t = self.handle.retsig.unwrap_or(Type::Void);
        let inst = InstData {
            block: self.block.index,
            kind: InstKind::Recur(data),
            typing: t,
        };
        self.push_inst(inst, args)
    }
}
