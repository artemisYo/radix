use crate::util::{JaggedVec, Key, KeyChain, KeyVec};
use std::marker::PhantomData;

// Contains structs that store the actual data

#[derive(Debug, Clone)]
pub enum Type {
    Int32,
}

// Containers
//   ... contain data

// Signatures as passed around
pub type SigSlice<'a> = &'a [Type];

pub struct Unit {
    // extra data stored contiguously, untyped,
    // use is determined by the instruction kind.
    // used to store stuff like branch arguments
    pub(crate) data: JaggedVec<Data, u32>,
    pub(crate) signatures: JaggedVec<Signature, Type>,
    pub(crate) blocks: KeyVec<Block, BlockData>,
    pub(crate) instructions: KeyVec<Instruction, InstData>,
    pub(crate) retsig: Option<Box<[Type]>>,
}

#[derive(Default)]
pub struct BlockData {
    pub(crate) signature: Signature,
    pub(crate) start: Instruction,
    pub(crate) end: Instruction,
}

pub(crate) enum InstData {
    FetchArg(usize),
    IConst(isize),
    Add([Instruction; 2]),
    Sub([Instruction; 2]),
    Less([Instruction; 2]),
    More([Instruction; 2]),
    Recur(Data),
    Terminator(TermData),
}
pub(crate) enum TermData {
    DoIf(Instruction),
    Branch(Block, Data),
}

// Indeces
//   These act as handles into the containers
//   used for associating and addressing stuff

// addresses extra data, index + length
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
pub struct Data(pub(crate) [u32; 2]);
// addresses a signature, index + length
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
pub struct Signature(pub(crate) [u32; 2]);
// addresses an instruction, index only
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
pub struct Instruction(pub(crate) u32);
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
pub struct Block(pub(crate) u32);
// stores some guards for builders
pub struct BlockHandle<Init, Sealed> {
    pub(crate) index: Block,
    pub(crate) _p: PhantomData<(Init, Sealed)>,
}

// Rest
//   These are some trait impls and other stuff
//   that can be ignored

impl Unit {
    pub fn new() -> Self {
        Self {
            data: JaggedVec::new(),
            signatures: JaggedVec::new(),
            blocks: KeyVec::new(),
            instructions: KeyVec::new(),
            retsig: None,
        }
    }
}

impl BlockData {
    pub(crate) fn new(sig: Signature) -> Self {
        Self {
            signature: sig,
            ..Default::default()
        }
    }
}

impl Key for Instruction {
    fn from(index: usize) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self(index.try_into().ok()?))
    }

    fn into(self) -> usize {
        self.0.try_into().unwrap()
    }
}

impl Block {
    pub const MAX: Self = Self(u32::MAX);
}
impl Key for Block {
    fn from(index: usize) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self(index.try_into().ok()?))
    }

    fn into(self) -> usize {
        self.0.try_into().unwrap()
    }
}

impl KeyChain for Signature {
    fn from(idx: usize, len: usize) -> Option<Self>
    where
        Self: Sized,
    {
        let idx = idx.try_into().ok()?;
        let len = len.try_into().ok()?;
        Some(Self([idx, len]))
    }

    fn into(self) -> (usize, usize) {
        let [idx, len] = self.0;
        (idx as usize, len as usize)
    }
}

impl KeyChain for Data {
    fn from(idx: usize, len: usize) -> Option<Self>
    where
        Self: Sized,
    {
        let idx = idx.try_into().ok()?;
        let len = len.try_into().ok()?;
        Some(Self([idx, len]))
    }

    fn into(self) -> (usize, usize) {
        let [idx, len] = self.0;
        (idx as usize, len as usize)
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.0)
    }
}
impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "b{}", self.0)
    }
}
