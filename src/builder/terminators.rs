use super::*;

/// Terminators for the Builder
impl<'a> Builder<'a> {
    pub(crate) fn ret_inner(&mut self, args: &[Instruction]) {
        self.register_dd(args.into_iter().cloned());
        let data = self.handle.data.push_slice(args);
        let inst = InstData {
            block: self.block.index,
            kind: InstKind::Terminator(TermData::Branch(Block::MAX, data)),
            typing: Type::Void,
        };
        self.handle
            .instructions
            .push(inst);
    }
    pub(crate) fn branch_inner<T>(&mut self, block: &BlockHandle<T>, args: &[Instruction]) {
        self.register_dd(args.into_iter().cloned());
        let data = self.handle.data.push_slice(args);
        let inst = InstData {
            block: self.block.index,
            kind: InstKind::Terminator(TermData::Branch(block.index, data)),
            typing: Type::Void,
        };
        self.handle
            .instructions
            .push(inst);
    }
    pub(crate) fn terminate(self) -> BlockHandle<True> {
        let index = self.block.index;
        BlockHandle {
            index,
            _p: PhantomData,
        }
    }
    pub fn ret(mut self, args: &[Instruction]) -> BlockHandle<True> {
        self.ret_inner(args);
        self.terminate()
    }
    pub fn branch<T>(
        mut self,
        block: &BlockHandle<T>,
        args: &[Instruction],
    ) -> BlockHandle<True> {
        self.branch_inner(block, args);
        self.terminate()
    }
    pub fn do_if(mut self, condition: Instruction) -> IfBuilder<'a, False> {
        self.register_dd([condition]);
        let inst = InstData {
            block: self.block.index,
            kind: InstKind::Terminator(TermData::DoIf(condition)),
            typing: Type::Void,
        };
        self.handle
            .instructions
            .push(inst);
        IfBuilder {
            builder: self,
            _p: PhantomData,
        }
    }
}

/// IfBuilder is received by the Builder itself
/// and used to ensure the if statement is built
/// correctly.
/// Basically inherits the terminator functions
/// from the Builder
pub struct IfBuilder<'a, Done> {
    pub(crate) builder: Builder<'a>,
    pub(crate) _p: PhantomData<Done>,
}

impl<'a> IfBuilder<'a, False> {
    pub fn ret(mut self, args: &[Instruction]) -> IfBuilder<'a, True> {
        self.builder.ret_inner(args);
        self.next()
    }
    pub fn branch<T>(
        mut self,
        block: &BlockHandle<T>,
        args: &[Instruction],
    ) -> IfBuilder<'a, True> {
        self.builder.branch_inner(block, args);
        self.next()
    }
    fn next(self) -> IfBuilder<'a, True> {
        IfBuilder {
            builder: self.builder,
            _p: PhantomData,
        }
    }
}

impl<'a> IfBuilder<'a, True> {
    pub fn ret(self, args: &[Instruction]) -> BlockHandle<True> {
        self.builder.ret(args)
    }
    pub fn branch<T>(
        self,
        block: &BlockHandle<T>,
        args: &[Instruction],
    ) -> BlockHandle<True> {
        self.builder.branch(block, args)
    }
}
