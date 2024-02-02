use crate::data::{
    Block, BlockData, BlockHandle, InstData, Instruction, SigSlice, TermData, Type, Unit,
};
use crate::util::{False, True};
use std::collections::BTreeSet;
use std::marker::PhantomData;

mod instructions;
mod terminators;

/// Builder
pub struct Builder<'a, Seal> {
    pub(crate) block: BlockHandle<False, Seal>,
    pub(crate) handle: &'a mut Unit,
}

// Stuff related to the builder pattern used
// for writing instructions and stuff

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
        self.backlinks.push(BTreeSet::new());
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
        F: FnOnce(Builder<Seal>) -> BlockHandle<True, Seal>,
    {
        let idx = b.index;
        self.blocks[idx].start = self.instructions.next_idx();
        let out = f(Builder {
            block: b,
            handle: self,
        });
        self.blocks[idx].end = self.instructions.current_idx();
        out
    }
    /// Finalizes the unit.
    /// Checks it for consistency, prevents further
    /// modification by the user.
    pub fn finalize(mut self, sig: Box<[Type]>) -> Self {
        self.retsig = Some(sig);
        // run a function to check the validity of the ir here
        self.remove_unused();
        self
    }
}
