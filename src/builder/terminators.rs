use super::*;

/// Terminators for the Builder
impl<'a, Seal> Builder<'a, Seal> {
    pub(crate) fn ret_inner(&mut self, args: &[Instruction]) {
        let data = self.handle.data.push_iter(args.into_iter().map(|a| a.0));
        self.handle
            .instructions
            .push(InstData::Terminator(TermData::Branch(Block::MAX, data)));
    }
    pub(crate) fn branch_inner<T>(&mut self, block: &BlockHandle<T, False>, args: &[Instruction]) {
        let data = self.handle.data.push_iter(args.into_iter().map(|a| a.0));
        self.handle
            .instructions
            .push(InstData::Terminator(TermData::Branch(block.index, data)));
    }
    pub(crate) fn terminate(self) -> BlockHandle<True, Seal> {
        let index = self.block.index;
        BlockHandle {
            index,
            _p: PhantomData,
        }
    }
    pub fn ret(mut self, args: &[Instruction]) -> BlockHandle<True, Seal> {
		self.ret_inner(args);
		self.terminate()
    }
    pub fn branch<T>(mut self, block: &BlockHandle<T, False>, args: &[Instruction]) -> BlockHandle<True, Seal> {
		self.branch_inner(block, args);
		self.terminate()
    }
    pub fn do_if(self, condition: Instruction) -> IfBuilder<'a, False, Seal> {
        self.handle
            .instructions
            .push(InstData::Terminator(TermData::DoIf(condition)));
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
pub struct IfBuilder<'a, Done, Seal> {
    pub(crate) builder: Builder<'a, Seal>,
    pub(crate) _p: PhantomData<Done>,
}

impl<'a, Seal> IfBuilder<'a, False, Seal> {
    pub fn ret(mut self, args: &[Instruction]) -> IfBuilder<'a, True, Seal> {
        self.builder.ret_inner(args);
        self.next()
    }
    pub fn branch<T>(
        mut self,
        block: &BlockHandle<T, False>,
        args: &[Instruction],
    ) -> IfBuilder<'a, True, Seal> {
        self.builder.branch_inner(block, args);
        self.next()
    }
    fn next(self) -> IfBuilder<'a, True, Seal> {
        IfBuilder {
            builder: self.builder,
            _p: PhantomData,
        }
    }
}

impl<'a, Seal> IfBuilder<'a, True, Seal> {
    pub fn ret(self, args: &[Instruction]) -> BlockHandle<True, Seal> {
        self.builder.ret(args)
    }
    pub fn branch<T>(
        self,
        block: &BlockHandle<T, False>,
        args: &[Instruction],
    ) -> BlockHandle<True, Seal> {
        self.builder.branch(block, args)
    }
}
