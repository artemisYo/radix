use crate::data::{Block, InstData, Instruction, TermData, Unit};

pub(crate) struct InstIter([Instruction; 2]);
impl std::iter::Iterator for InstIter {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0[0] > self.0[1] {
            return None;
        }
        let out = self.0[0];
        self.0[0].0 += 1;
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
            write!(out, "---b{}{:?}:\n", bi, &self.signatures[b.signature]).unwrap();
            for i in b.start.until(b.end) {
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
            write!(out, "---return{:?}\n", s).unwrap();
        }
        out
    }
}

impl InstData {
    fn human_format(&self, unit: &Unit) -> String {
        if let InstData::Terminator(t) = self {
            t.human_format(unit)
        } else {
            format!(
                "= {}",
                &match self {
                    InstData::Tombstone => "_".to_string(),
                    InstData::FetchArg(i) => format!("fetchArg [{i}]"),
                    InstData::IConst(i) => format!("const {i}"),
                    InstData::Add([a, b]) => format!("add {a}, {b}"),
                    InstData::Sub([a, b]) => format!("sub {a}, {b}"),
                    InstData::Less([a, b]) => format!("less {a}, {b}"),
                    InstData::More([a, b]) => format!("more {a}, {b}"),
                    InstData::Recur(d) => format!("recur {:?}", &unit.data[*d]),
                    InstData::Terminator(_) => unreachable!(),
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
