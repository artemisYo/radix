use std::{mem::ManuallyDrop, marker::PhantomData};

type Sig = Vec<Type>;

pub struct Unit {
    blocks: Vec<BlockData>,
}

impl Unit {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }
    pub fn new_block(&mut self, sig: Sig) -> Block<Uninit> {
        let idx = self.blocks.len();
        self.blocks.push(BlockData::new(sig));
        Block { index: idx, _p: PhantomData }
    }
    pub fn seal(&self, b: Block<T>)
    pub fn with_block<F>(&mut self, b: Block<Uninit>, f: F) -> Block<Init>
    where F: FnOnce(BlockHandle) -> Block<Init> {
		f(BlockHandle(&mut self.blocks[b.index]))
    }
}

pub struct BlockData {
    signature: Sig,
    instructions: Vec<Inst>,
}
impl BlockData {
    fn new(sig: Sig) -> Self {
        Self { signature: sig, instructions: Vec::new() }
    }
}

pub struct Uninit;
pub struct Init;
pub struct Sealed;
pub struct Block<T> {
    index: usize,
    _p: PhantomData<T>
}

pub struct BlockHandle<'a>(&'a mut BlockData);

pub enum Type {}
pub enum Inst {}
