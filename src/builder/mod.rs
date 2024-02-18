use crate::data::{
    Block, BlockData, BlockHandle, InstData, InstKind, Instruction, SigSlice, TermData, Type, Unit,
};
use crate::util::{False, True};
use std::marker::PhantomData;

mod instructions;
mod terminators;

/// Builder
pub struct Builder<'a> {
    pub(crate) block: BlockHandle<False>,
    pub(crate) handle: &'a mut Unit,
}

// Stuff related to the builder pattern used
// for writing instructions and stuff
impl Unit {
    /// Creates a block and returns it's index.
    /// The block is bound to the given signature,
    /// however does not contain any instructions nor
    /// any entry points, unless it's the first block.
    pub fn new_block(&mut self, sig: SigSlice) -> BlockHandle<False> {
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
    pub fn with_block<F>(&mut self, b: BlockHandle<False>, f: F) -> BlockHandle<True>
    where
        F: FnOnce(Builder) -> BlockHandle<True>,
    {
        let idx = b.index;
        self.blocks[idx].inst_range[0] = self.instructions.next_idx();
        let out = f(Builder {
            block: b,
            handle: self,
        });
        self.blocks[idx].inst_range[1] = self.instructions.next_idx();
        out
    }
    /// Finalizes the unit.
    /// Checks it for consistency
    pub fn finalize(mut self, sig: Type) -> Self {
        self.retsig = Some(sig);
        // run a function to check the validity of the ir here
        self.check_dependencies();
        self.remove_unused();
        self.annotate_liveness();
        self
    }
}
