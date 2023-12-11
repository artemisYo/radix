use std::marker::PhantomData;

use crate::{MakeKey, MakeRange, utils::{Single, Multiple, KeyVec}};

MakeKey!(InstIdx, u32);
MakeKey!(BlockIdx, u16);
MakeRange!(SigIdx, u16, u8);
MakeRange!(ArgIdx, u16, u8);

struct InstStep {
    head: u32,
    end: u32,
}
impl InstIdx {
    fn until(self, end: Self) -> InstStep {
        InstStep { head: self.0, end: end.0 }
    }
}
impl Iterator for InstStep {
    type Item = InstIdx;

    fn next(&mut self) -> Option<Self::Item> {
        if self.head > self.end {return None;}
        let out = InstIdx::from(self.head as usize);
        self.head += 1;
        Some(out)
    }
}

pub trait Finality {}
#[derive(Debug)]
pub struct InConstruction;
impl Finality for InConstruction {}
#[derive(Debug)]
pub struct Finalized;
impl Finality for Finalized {}

// one unit --- corresponds to a single function
#[derive(Debug)]
pub struct Unit<F: Finality> {
    _phantom: PhantomData<F>,
    pub(self) blocks: KeyVec<Single, BlockIdx, Block>,
    pub(self) insts: KeyVec<Single, InstIdx, Instruction>,
    pub(self) sigs: KeyVec<Multiple, SigIdx, Type>,
    pub(self) extra_args: KeyVec<Multiple, ArgIdx, u32>
}

impl Unit<Finalized> {
    pub fn human_format(&self) -> String {
        let mut out = String::new();
        for (i, b) in self.blocks
            .iter()
            .take(self.blocks.len() - 1)
            .enumerate() {
            out.push_str(&format!(".b{} {:?}\n", i, &self.sigs[b.sig]));
            for inst in b.start.until(b.end) {
                out.push_str(&format!("    {}\n", self.insts[inst]));
            }
        }
        out.push_str(
            &format!("return: {:?}\n",
                     &self.sigs[self.blocks.last().unwrap().sig]));
        out
    }
}

impl Unit<InConstruction> {
    pub fn new(signature: &[Type]) -> Self {
        let mut out = Self {
            _phantom: PhantomData,
            blocks: KeyVec::<Single, BlockIdx, Block>::new(),
            insts: KeyVec::<Single, InstIdx, Instruction>::new(),
            sigs: KeyVec::<Multiple, SigIdx, Type>::new(),
            extra_args: KeyVec::<Multiple, ArgIdx, u32>::new(),
        };
        // entry point block
        let sig = out.sigs.append(signature);
        out.blocks.push(Block {sig, start: 0.into(), end: 0.into()});
        out
    }
    pub fn finalize(mut self, return_sig: &[Type]) -> Unit<Finalized> {
        let sig = self.sigs.append(return_sig);
        let idx = (u32::MAX as usize).into();
        // return point block
        self.blocks.push(Block {sig, start: idx, end: idx});
        Unit {
            _phantom: PhantomData,
            blocks: self.blocks,
            insts: self.insts,
            sigs: self.sigs,
            extra_args: self.extra_args
        }
    }
    pub fn push(&mut self) -> InstStack {InstStack(self)}
    pub fn new_block(&mut self, sig: &[Type]) -> BlockIdx {
        assert!(self.insts.last().unwrap().is_term());
        let sig = self.sigs.append(sig);
        self.blocks.push(Block {sig, start: self.insts.last_key(), end: 0.into()})
    }
}

#[derive(Debug)]
pub struct InstStack<'a>(&'a mut Unit<InConstruction>);
impl<'a> InstStack<'a> {
    pub fn terminate(&mut self) {
        self.0.blocks.last_mut().unwrap().end = self.0.insts.last_key();
    }
    pub fn iconst(&mut self, int: usize) -> InstIdx {
        match u32::try_from(int) {
            Ok(i) => self.0.insts.push(Instruction::IntConst(i)),
            Err(_) => {
                let i = self.0.extra_args.append(&[
                    (int >> 32) as u32,
                    (int & 0xFFFFFFFF) as u32
                ]);
                self.0.insts.push(Instruction::BIntConst(i))
            }
        }
    }
    pub fn ret(&mut self, value: InstIdx) {
        self.0.insts.push(Instruction::Ret(value));
        self.terminate();
    }
}

#[derive(Debug)]
pub struct Block {
    sig: SigIdx,
    start: InstIdx,
    end: InstIdx,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {Int8, Int16, Int32, Int64}
#[derive(Debug)]
pub enum Instruction {
    IntConst(u32),
    // used for ints > 32 bits
    BIntConst(ArgIdx),
    Ret(InstIdx)
}
impl Instruction {
    fn is_term(&self) -> bool {
        use Instruction::*;
        match self {
            Ret(_) => true,
            _ => false
        }
    }
}
impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Instruction::*;
        match self {
            IntConst(i) => f.write_str(&format!("const {}", i)),
            BIntConst(a) => f.write_str(&format!("const &{}", a.0)),
            Ret(v) => f.write_str(&format!("return @{}", v.0)),
        }
    }
}
