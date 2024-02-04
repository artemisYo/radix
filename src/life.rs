use crate::data::{Block, Instruction, Unit};

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
        let acc = vec![Duration::Leak; cap];
        for (_n, i) in blockdata.start.until(blockdata.end).enumerate() {
            let _instdata = &self.instructions[i];
            todo!()
        }
        acc
    }
}
