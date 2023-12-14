use super::*;

impl Unit<'_, InConstruction> {
    pub fn entry_block(&self) -> BlockHandle<()> {
		BlockHandle {index: 0.into(), handle: ()}
    }
	pub fn new_block(&mut self, sig: &[Type]) -> BlockHandle<()> {
        let sig = self.sigs.append(sig);
        let index = self.blocks.push(Block {sig, start: 0.into(), end: 0.into()});
		BlockHandle {index, handle: ()}
	}
	pub fn switch_to(&mut self, block: &BlockHandle<()>) -> BlockHandle<&mut Self> {
		BlockHandle {
			index: block.index,
			handle: self
		}
	}
}

pub trait BuilderHandleable {}
impl BuilderHandleable for () {}
impl BuilderHandleable for &mut Unit<'_, InConstruction> {}

pub struct BlockHandle<T: BuilderHandleable> {
	pub(super) index: BlockIdx,
	pub(super) handle: T
}
impl<T: BuilderHandleable> From<&BlockHandle<T>> for BlockIdx {
    fn from(value: &BlockHandle<T>) -> Self {
        value.index
    }
}

pub struct InstBuilder<'a, 'b: 'a, 'c: 'b>(pub(super) &'a mut BlockHandle<&'b mut Unit<'c, InConstruction>>);
pub struct TerminatorBuilder<'a, 'b>(pub(super) BlockHandle<&'a mut Unit<'b, InConstruction>>);
impl Drop for TerminatorBuilder<'_, '_> {
    fn drop(&mut self) {
        self.0.handle.blocks[self.0.index].end = self.0.handle.insts.last_key();
    }
}

impl<'a, 'b: 'a> BlockHandle<&'a mut Unit<'b, InConstruction>> {
    pub fn push<'c>(&'c mut self) -> InstBuilder<'c, 'a, 'b> {
        let block = &mut self.handle.blocks[self.index];
        assert!(block.end.0 == 0);
        if block.start.0 == 0 {
			block.start = self.handle.insts.new_key();
        }
		InstBuilder(self)
    }
    pub fn terminate(self) -> TerminatorBuilder<'a, 'b> {
		TerminatorBuilder(self)
    }
}
