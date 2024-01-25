use crate::data::BlockData;
use crate::util::QuadVec;
use crate::{data::Block, data::InstData, data::Instruction, Unit};

impl Unit {
    // traverses width-first and fills out an adjacency matrix
    // where each row contains "true" for each directly previous block
    // also returns the order of traversal
    pub(crate) fn block_adjacency(&self) -> (Vec<u32>, QuadVec<bool>) {
        let mut out = QuadVec::filled(self.blocks.len(), false);
        let mut queue = vec![0];
        let mut counter = 0;
        while let Some(block) = queue.get(counter).cloned() {
            let mut col = out.col(block as usize);
            let blockdata = &self.blocks[Block(block)];
            let [next, pass] = blockdata.get_next(self);
            pass.map(|p| {
                col[p.0 as usize] = true;
                queue.push(p.0)
            });
            next.map(|p| {
                col[p.0 as usize] = true;
                queue.push(p.0)
            });
            counter += 1;
        }
        (queue, out)
    }
    pub(crate) fn remove_unused(&mut self) {
        let mut unused = vec![true; self.instructions.len()];
        let (order, _) = self.block_adjacency();
        // iterate all blocks in reversed width-first order
        // and mark any values appearing on the rhs of an inst
        // as used, if the inst itself is used
        for block in order.into_iter().rev().map(|i| Block(i)) {
            let blockdata = &self.blocks[block];
            for inst in blockdata.start.until(blockdata.end) {
                let [a, b] = self.instructions[inst].get_insts();
                if !unused[inst.0 as usize] {
                    a.map(|i| unused[i.0 as usize] = false);
                    b.map(|i| unused[i.0 as usize] = false);
                }
            }
        }
        for (i, _) in unused.into_iter().enumerate().filter(|(_, b)| *b) {
            self.instructions[Instruction(i as u32)] = InstData::Tombstone;
        }
    }
}

impl BlockData {
    // returns None for the return block index as it does not count as a block
    fn get_next(&self, unit: &Unit) -> [Option<Block>; 2] {
        let mut out = [unit.instructions[self.end].get_block(), None];
        if self.end.0 - self.start.0 < 2 {
            return out;
        }
        if let InstData::Terminator(_) = unit.instructions[Instruction(self.end.0 - 2)] {
            out = [
                unit.instructions[Instruction(self.end.0 - 1)].get_block(),
                unit.instructions[self.end].get_block(),
            ];
            return out;
        }
        out
    }
}

impl InstData {
    fn get_block(&self) -> Option<Block> {
        match self {
            Self::Terminator(crate::data::TermData::Branch(Block::MAX, _)) => None,
            Self::Terminator(crate::data::TermData::Branch(b, _)) => Some(*b),
            _ => None,
        }
    }
    fn get_insts(&self) -> [Option<Instruction>; 2] {
        match self {
            Self::Add([a, b]) | Self::Sub([a, b]) | Self::Less([a, b]) | Self::More([a, b]) => {
                [Some(*a), Some(*b)]
            }
            Self::Terminator(crate::data::TermData::DoIf(i)) => [Some(*i), None],
            _ => [None, None],
        }
    }
}
