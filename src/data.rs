use crate::util::{JaggedVec, Key, KeyChain, KeyVec};
use std::marker::PhantomData;

pub type Set<V> = std::collections::BTreeSet<V>;

// Contains structs that store the actual data

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Int32,
    Void,
}

#[derive(Debug)]
pub struct Settings {
    pub volatile: bool,
}
impl std::default::Default for Settings {
    fn default() -> Self {
        Self { volatile: true }
    }
}

// Containers
//   ... contain data

// Signatures as passed around
pub type SigSlice<'a> = &'a [Type];

pub struct Unit {
    pub settings: Settings,
    pub(crate) data: JaggedVec<Data, Instruction>,
    pub(crate) signatures: JaggedVec<Signature, Type>,
    pub(crate) blocks: KeyVec<Block, BlockData>,
    pub(crate) instructions: KeyVec<Instruction, InstData>,
    pub(crate) use_meta: KeyVec<UseSet, UseMetadata>,
    pub(crate) retsig: Option<Type>,
}

#[derive(Default)]
pub struct BlockData {
    pub(crate) dd: Set<Block>,
    pub(crate) signature: Signature,
    pub(crate) uset_start: UseSet,
    pub(crate) uset_end: UseSet,
    pub(crate) inst_start: Instruction,
    pub(crate) inst_end: Instruction,
}

pub(crate) struct InstData {
    pub(crate) block: Block,
    pub(crate) typing: Type,
    pub(crate) kind: InstKind,
}
pub(crate) enum InstKind {
    Tombstone,
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

pub(crate) struct UseMetadata {
    pub(crate) used: Instruction,
    pub(crate) location: Instruction,
    pub(crate) is_final: bool,
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy, Debug)]
pub struct Block(pub(crate) u32);
// stores some guards for builders
pub struct BlockHandle<Init> {
    pub(crate) index: Block,
    pub(crate) _p: PhantomData<Init>,
}
// stored in blockdata to associate a block with
// the instruction use metadata
#[derive(PartialEq, Eq, Default, Clone, Copy, Debug)]
pub struct UseSet(pub(crate) u32);

// Rest
//   These are some trait impls and other stuff
//   that can be ignored

impl Unit {
    pub fn new() -> Self {
        Self {
            settings: Settings::default(),
            data: JaggedVec::new(),
            signatures: JaggedVec::new(),
            blocks: KeyVec::new(),
            instructions: KeyVec::new(),
            use_meta: KeyVec::new(),
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

impl Key for UseSet {
    fn from(idx: usize) -> Option<Self>
    where
        Self: Sized,
    {
        let idx = idx.try_into().ok()?;
        Some(Self(idx))
    }

    fn into(self) -> usize {
        let idx = self.0;
        idx as usize
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.0)
    }
}
impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.0)
    }
}
impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "b{}", self.0)
    }
}
