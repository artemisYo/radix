use crate::util::{Key, KeyVec};
use std::marker::PhantomData;

pub type Set<V> = std::collections::BTreeSet<V>;
pub type Map<K, V> = std::collections::BTreeMap<K, V>;

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
    pub(crate) data: KeyVec<DataPart, Instruction>,
    pub(crate) signatures: KeyVec<SignaturePart, Type>,
    pub(crate) blocks: KeyVec<Block, BlockData>,
    pub(crate) instructions: KeyVec<Instruction, InstData>,
    pub liveness: Map<(Block, Instruction), LiveData>,
    pub(crate) retsig: Option<Type>,
}

#[derive(Default)]
pub struct BlockData {
    pub(crate) signature: [SignaturePart; 2],
    pub(crate) inst_range: [Instruction; 2],
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
    Recur([DataPart; 2]),
    Terminator(TermData),
}
pub(crate) enum TermData {
    DoIf(Instruction),
    Branch(Block, [DataPart; 2]),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LiveData {
    Alive,
    Partial(Instruction),
}

// Indeces
//   These act as handles into the containers
//   used for associating and addressing stuff

// addresses extra data, index + length
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
pub struct DataPart(pub(crate) u32);
// addresses a signature, index + length
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
pub struct SignaturePart(pub(crate) u32);
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

// Rest
//   These are some trait impls and other stuff
//   that can be ignored

impl Unit {
    pub fn new() -> Self {
        Self {
            settings: Settings::default(),
            data: KeyVec::new(),
            signatures: KeyVec::new(),
            blocks: KeyVec::new(),
            instructions: KeyVec::new(),
            liveness: Map::new(),
            retsig: None,
        }
    }
}

impl BlockData {
    pub(crate) fn new(sig: [SignaturePart; 2]) -> Self {
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
        self.0 as usize
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
        self.0 as usize
    }
}

impl Key for SignaturePart {
    fn from(idx: usize) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self(idx.try_into().ok()?))
    }

    fn into(self) -> usize {
        self.0 as usize
    }
}

impl Key for DataPart {
    fn from(idx: usize) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self(idx.try_into().ok()?))
    }

    fn into(self) -> usize {
        self.0 as usize
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
