use crate::data::{
    Block, BlockData, BlockHandle, InstData, Instruction, SigSlice, TermData,
    Unit, Type
};
use crate::util::{False, True};
use std::marker::PhantomData;

// Stuff related to the builder pattern used
// for writing instructions and stuff

/// Builder Base.
pub struct Builder<'a, Seal> {
    pub(crate) block: BlockHandle<False, Seal>,
    pub(crate) handle: &'a mut Unit,
}

impl<'a, Seal> Builder<'a, Seal> {
    pub(crate) fn ret(&mut self, args: &[Instruction]) {
        let data = self.handle.data.push_iter(args.into_iter().map(|a| a.0));
        self.handle
            .instructions
            .push(InstData::Terminator(TermData::Branch(Block::MAX, data)));
    }
    pub(crate) fn branch<T>(&mut self, block: &BlockHandle<T, False>, args: &[Instruction]) {
        let data = self.handle.data.push_iter(args.into_iter().map(|a| a.0));
        self.handle
            .instructions
            .push(InstData::Terminator(TermData::Branch(block.index, data)));
    }
    pub(crate) fn do_if(&mut self, condition: Instruction) {
        self.handle
            .instructions
            .push(InstData::Terminator(TermData::DoIf(condition)));
    }
    pub(crate) fn terminate(self) -> BlockHandle<True, Seal> {
        let index = self.block.index;
        BlockHandle {
            index,
            _p: PhantomData,
        }
    }
}

/// Builder that is passed to a closure
/// in Unit::with_block.
pub struct BlockBuilder<'a, Seal>(pub(crate) Builder<'a, Seal>);

/// Terminates the block.
/// This disallows modifying the instruction
/// content of the block, but does return it
/// as it can still be used to branch to and
/// was consumed before.
impl<'a, Seal> BlockBuilder<'a, Seal> {
    pub fn ret(mut self, args: &[Instruction]) -> BlockHandle<True, Seal> {
        self.0.ret(args);
        self.0.terminate()
    }
    pub fn branch<T>(
        mut self,
        block: &BlockHandle<T, False>,
        args: &[Instruction],
    ) -> BlockHandle<True, Seal> {
        self.0.branch(block, args);
        self.0.terminate()
    }
    /// Returns an IfBuilder, which ensures the terminator is built correctly
    /// on the type level.
    pub fn do_if(mut self, condition: Instruction) -> IfBuilder<'a, False, Seal> {
        self.0.do_if(condition);
        IfBuilder {
            builder: self.0,
            _p: PhantomData,
        }
    }
}

pub struct IfBuilder<'a, Done, Seal> {
    builder: Builder<'a, Seal>,
    _p: PhantomData<Done>,
}

impl<'a, Seal> IfBuilder<'a, False, Seal> {
    pub fn ret(mut self, args: &[Instruction]) -> IfBuilder<'a, True, Seal> {
        self.builder.ret(args);
        self.next()
    }
    pub fn branch<T>(
        mut self,
        block: &BlockHandle<T, False>,
        args: &[Instruction],
    ) -> IfBuilder<'a, True, Seal> {
        self.builder.branch(block, args);
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
    pub fn ret(mut self, args: &[Instruction]) -> BlockHandle<True, Seal> {
        self.builder.ret(args);
        self.builder.terminate()
    }
    pub fn branch<T>(
        mut self,
        block: &BlockHandle<T, False>,
        args: &[Instruction],
    ) -> BlockHandle<True, Seal> {
        self.builder.branch(block, args);
        self.builder.terminate()
    }
}

impl<Init> BlockHandle<Init, False> {
    /// Seals the block.
    /// This disallows branching to this block.
    pub fn seal(self) -> BlockHandle<Init, True> {
        BlockHandle {
            index: self.index,
            _p: PhantomData,
        }
    }
}

impl Unit {
    /// Creates a block and returns it's index.
    /// The block is bound to the given signature,
    /// however does not contain any instructions nor
    /// any entry points, unless it's the first block.
    pub fn new_block(&mut self, sig: SigSlice) -> BlockHandle<False, False> {
        let idx = self.blocks.next_idx();
        let sig_idx = self.signatures.push_slice(sig);
        self.blocks.push(BlockData::new(sig_idx));
        BlockHandle {
            index: idx,
            _p: PhantomData,
        }
    }
    /// Gives access to a block.
    /// It's used to insert instructions into a block.
    /// Can only be used once and the closure needs
    /// to return the updated form of the block
    /// obtained by inserting a terminator.
    pub fn with_block<Seal, F>(
        &mut self,
        b: BlockHandle<False, Seal>,
        f: F,
    ) -> BlockHandle<True, Seal>
    where
        F: FnOnce(BlockBuilder<Seal>) -> BlockHandle<True, Seal>,
    {
        let idx = b.index;
        self.blocks[idx].start = self.instructions.next_idx();
        let out = f(BlockBuilder(Builder {
            block: b,
            handle: self,
        }));
        self.blocks[idx].end = self.instructions.current_idx();
        out
    }
    /// Finalizes the unit.
    /// Checks it for consistency, prevents further
    /// modification by the user.
    pub fn finalize(mut self, sig: Box<[Type]>) -> Self {
        self.retsig = Some(sig);
        // run a function to check the validity of the ir here
        self
    }
}
