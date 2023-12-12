use std::marker::PhantomData;

use crate::{MakeKey, MakeRange, utils::{Single, Multiple, KeyVec}};
mod builder;
pub use builder::{Builder, ret};

MakeKey!(FuncRef, u32);
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
pub struct Unit<'a, F: Finality> {
    _phantom: PhantomData<F>,
    pub(self) funcs: KeyVec<Single, FuncRef, &'a [Type]>,
    pub(self) blocks: KeyVec<Single, BlockIdx, Block>,
    pub(self) insts: KeyVec<Single, InstIdx, Instruction>,
    pub(self) sigs: KeyVec<Multiple, SigIdx, Type>,
    pub(self) extra_args: KeyVec<Multiple, ArgIdx, u32>
}

impl<'a> Unit<'a, Finalized> {
    pub fn human_format(&self) -> String {
        let mut out = String::new();
        for (i, b) in self.blocks
            .iter()
            .take(self.blocks.len() - 1)
            .enumerate() {
            out.push_str(&format!(".b{} {:?}\n", i, &self.sigs[b.sig]));
            for inst in b.start.until(b.end) {
                out.push_str(&format!("    @{}: {}\n", inst.0, self.insts[inst].human_format(&self.extra_args, &self.funcs)));
            }
        }
        out.push_str(
            &format!("return: {:?}\n",
                     &self.sigs[self.blocks.last().unwrap().sig]));
        out
    }
}

impl<'a> Unit<'a, InConstruction> {
    fn new(signature: &'a [Type]) -> Self {
        let mut out = Self {
            _phantom: PhantomData,
            funcs: KeyVec::<Single, FuncRef, &'a [Type]>::new(),
            blocks: KeyVec::<Single, BlockIdx, Block>::new(),
            insts: KeyVec::<Single, InstIdx, Instruction>::new(),
            sigs: KeyVec::<Multiple, SigIdx, Type>::new(),
            extra_args: KeyVec::<Multiple, ArgIdx, u32>::new(),
        };
        // register self
        out.funcs.push(signature);
        // entry point block
        let sig = out.sigs.append(signature);
        out.blocks.push(Block {sig, start: 0.into(), end: 0.into()});
        out
    }
    fn finalize(mut self, return_sig: &'a [Type]) -> Unit<Finalized> {
        let sig = self.sigs.append(return_sig);
        let idx = (u32::MAX as usize).into();
        // return point block
        self.blocks.push(Block {sig, start: idx, end: idx});
        Unit {
            _phantom: PhantomData,
            funcs: self.funcs,
            blocks: self.blocks,
            insts: self.insts,
            sigs: self.sigs,
            extra_args: self.extra_args
        }
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
    // mostly used for deleting instructions
    Nop,
    FetchArg(u32),
    IntConst([u8;8]),
    Add(InstIdx, InstIdx),
    Sub(InstIdx, InstIdx),
    Mult(InstIdx, InstIdx),
    Div(InstIdx, InstIdx),
    Less(InstIdx, InstIdx),
    More(InstIdx, InstIdx),
    Equal(InstIdx, InstIdx),
    Call(FuncRef, ArgIdx),
    // used for ints > 32 bits
    DoIf(InstIdx),
    Branch(BlockIdx, ArgIdx),
}
impl Instruction {
    fn is_term(&self) -> bool {
        use Instruction::*;
        match self {
            Branch(_, _) => true,
            _ => false
        }
    }
}
impl Instruction {
    fn human_format(
        &self,
        args: &KeyVec<Multiple, ArgIdx, u32>,
        funcs: &KeyVec<Single, FuncRef, &[Type]>,
    ) -> String {
        use Instruction::*;
        match self {
            Nop => "".to_string(),
            FetchArg(n) => format!("blockArg[{}]", n),
            IntConst(i) => format!("const {:?}", i),
    		Add(a, b) => format!("add @{} @{}", a.0, b.0),
    		Sub(a, b) => format!("sub @{} @{}", a.0, b.0),
    		Mult(a, b) => format!("mult @{} @{}", a.0, b.0),
    		Div(a, b) => format!("div @{} @{}", a.0, b.0),
    		Less(a, b) => format!("less @{} @{}", a.0, b.0),
    		More(a, b) => format!("more @{} @{}", a.0, b.0),
    		Equal(a, b) => format!("equal @{} @{}", a.0, b.0),
    		Call(f, a) => format!("call {:?} {:?}", &funcs[*f], &args[*a]),
            DoIf(i) => format!("if @{}", i.0),
            Branch(b, a) => {
                let u16::MAX = b.0 else {
                	return format!("branch b{} {:?}", b.0, &args[*a]);
                };
                format!("branch return {:?}", &args[*a])
            },
        }
    }
}
