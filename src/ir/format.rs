use super::*;

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
