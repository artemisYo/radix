use crate::data::{Unit, Block, Instruction};

#[derive(Clone, Copy)]
pub enum Duration {
	Leak,
	Until(Instruction),
}

impl Unit {
    // Lifetimes if not found within the current block are counted as leaked
    // as any unused instructions should be removed in some prior steps.
    fn block_lifetimes(&self, block: Block) -> Vec<Duration> {
        let blockdata = &self.blocks[block];
        let cap = (blockdata.end.0 - blockdata.start.0) as usize;
        let mut acc = vec![Duration::Leak; cap];
        for (n, i) in blockdata.start.until(blockdata.end).enumerate() {
			let instdata = &self.instructions[i];
			todo!()
        }
    	acc
    }
}
