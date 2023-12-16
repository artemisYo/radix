mod instructions;
mod blocks;
mod format;

pub use instructions::ret;
use std::marker::PhantomData;
use crate::{MakeKey, MakeRange, utils::{Single, Multiple, KeyVec}};
use instructions::Instruction;

MakeKey!(FuncRef, u32);
MakeKey!(InstIdx, u32);
MakeKey!(BlockIdx, u16);
MakeRange!(SigIdx, u16, u8);
MakeRange!(ArgIdx, u16, u8);

#[derive(Debug)] pub struct InConstruction;
#[derive(Debug)] pub struct Finalized;
pub trait Finality {}
impl Finality for InConstruction {}
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

fn lifetime_annot(unit: &mut Unit<'_, Finalized>) {
    let mut last_used = KeyVec::<Single, InstIdx, InstIdx>::new();
    for (idx, i) in unit.insts.iter().enumerate()
        .map(|(idx, i)| (Into::<InstIdx>::into(idx), i)) {
        match i {
            Instruction::Sub(a, b)
			| Instruction::Add(a, b) => {
    			last_used[*a] = idx;
    			last_used[*b] = idx;
			}
			Instruction::Call(_, a) => {
				let args = &unit.extra_args[*a];
				for a in args {
					last_used[(*a as usize).into()] = idx;
				}
			}
			_ => todo!()
        }
    }
    // then iterate each instruction, giving it a register
    // then when there are no more registers available
    // check whether any of the registers contain a dead instruction
    // if so then use that
    // otherwise spill
}

impl<'a> Unit<'a, InConstruction> {
    pub fn new(sig: &'a [Type]) -> Self {
        let mut out = Self {
            _phantom: PhantomData,
            funcs: KeyVec::<Single, FuncRef, &'a [Type]>::new(),
            blocks: KeyVec::<Single, BlockIdx, Block>::new(),
            insts: KeyVec::<Single, InstIdx, Instruction>::new(),
            sigs: KeyVec::<Multiple, SigIdx, Type>::new(),
            extra_args: KeyVec::<Multiple, ArgIdx, u32>::new(),
        };
        // padding, as 0 InstIdx is used as a None value
        out.insts.push(Instruction::Nop);
        // entry block
        out.new_block(sig);
        out.funcs.push(sig);
        out
    }
    pub fn finalize(mut self, return_sig: &'a [Type]) -> Unit<Finalized> {
        let sig = self.sigs.append(return_sig);
        let idx = (u32::MAX as usize).into();
        // return point block
        self.blocks.push(Block::new(sig));
        self.blocks.last_mut().unwrap().start = idx;
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
impl Block {
	fn new(sig: SigIdx) -> Self {
		Self {sig, start: 0.into(), end: 0.into()}
	}
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {Int8, Int16, Int32, Int64}
