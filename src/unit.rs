use std::mem::ManuallyDrop;

type Sig = Vec<Type>;

pub struct Unit {
    blocks: Vec<BlockData>,
}

impl Unit {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }
    pub fn new_block(&mut self, sig: Sig) -> Block<()> {
        let idx = self.blocks.len();
        self.blocks.push(BlockData::new(sig));
        ManuallyDrop::new(BlockHandle { index: idx, handle: () })
    }
    pub fn switch_block(&mut self, b: Block<()>) -> Block<&mut Self> {
        ManuallyDrop::new(BlockHandle { index: b.index, handle: self })
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

pub type Block<T> = ManuallyDrop<BlockHandle<T>>;

pub struct BlockHandle<T> {
    index: usize,
    handle: T
}

pub enum Type {}
pub enum Inst {}
