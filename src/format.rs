use crate::data::{Block, InstData, InstKind, Instruction, TermData, Unit};

pub(crate) struct InstIter([Instruction; 2]);
impl std::iter::Iterator for InstIter {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0[0] >= self.0[1] {
            return None;
        }
        let out = self.0[0];
        self.0[0].0 += 1;
        Some(out)
    }
}
impl std::iter::DoubleEndedIterator for InstIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.0[0] >= self.0[1] {
            return None;
        }
        let out = self.0[0];
        self.0[1].0 -= 1;
        Some(out)
    }
}
impl Instruction {
    pub(crate) fn until(self, end: Self) -> InstIter {
        InstIter([self, end])
    }
}

impl Unit {
    pub fn human_format(&self) -> String {
        use std::fmt::Write;
        let mut out: String = "".into();
        let mut inst_counter = 0;
        for (bi, b) in self.blocks.iter().enumerate() {
            write!(
                out,
                "---b{}{:?}:\n",
                bi, &self.signatures[b.signature],
            )
            .unwrap();
            for i in b.inst_range[0].until(b.inst_range[1]) {
                write!(
                    out,
                    "|\t@{} {}\n",
                    inst_counter,
                    self.instructions[i].human_format(self)
                )
                .unwrap();
                inst_counter += 1;
            }
        }
        if let Some(s) = &self.retsig {
            write!(out, "---return({:?})\n", s).unwrap();
        }
        out
    }
}

impl InstData {
    fn human_format(&self, unit: &Unit) -> String {
        self.kind.human_format(unit)
    }
}

impl InstKind {
    fn human_format(&self, unit: &Unit) -> String {
        if let InstKind::Terminator(t) = self {
            t.human_format(unit)
        } else {
            format!(
                "= {}",
                &match self {
                    InstKind::Tombstone => "_".to_string(),
                    InstKind::FetchArg(i) => format!("fetchArg [{i}]"),
                    InstKind::IConst(i) => format!("const {i}"),
                    InstKind::Add([a, b]) => format!("add {a}, {b}"),
                    InstKind::Sub([a, b]) => format!("sub {a}, {b}"),
                    InstKind::Less([a, b]) => format!("less {a}, {b}"),
                    InstKind::More([a, b]) => format!("more {a}, {b}"),
                    InstKind::Recur(d) => format!("recur {:?}", &unit.data[*d]),
                    InstKind::Terminator(_) => unreachable!(),
                }
            )
        }
    }
}

impl TermData {
    fn human_format(&self, unit: &Unit) -> String {
        match self {
            TermData::DoIf(c) => format!(": if {c}"),
            TermData::Branch(Block::MAX, a) => format!(": ret {:?}", &unit.data[*a]),
            TermData::Branch(b, a) => format!(": br {b} {:?}", &unit.data[*a]),
        }
    }
}
