mod builder;
mod data;
mod instructions;
mod util;
mod format;

pub use data::Type;
pub use data::Unit;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fib() {
        let mut unit = Unit::new();
        let b0 = unit.new_block(&[Type::Int32]);
        let b1 = unit.new_block(&[]);
        // it can't tell n would be initialized in the closure
        let mut n = Default::default();
        unit.with_block(b0, |mut block| {
            n = block.fetch_arg(0);
            let two = block.iconst(2);
            let cond = block.less(n, two);
            block.do_if(cond).ret(&[n]).branch(&b1, &[])
        });
        unit.with_block(b1, |mut block| {
            let one = block.iconst(1);
            let a = block.sub(n, one);
            let fa = block.recurse(&[a]);
            let two = block.iconst(2);
            let b = block.sub(n, two);
            let fb = block.recurse(&[b]);
            let o = block.add(fa, fb);
            block.ret(&[o])
        });
        let ir = unit.finalize(Box::new([Type::Int32]));
        eprintln!("{}", ir.human_format());
    }
    #[test]
    fn construct() {
       let mut unit = Unit::new();
       let b0 = unit.new_block(&[]);
       unit.with_block(b0, |mut block| {
           let p = block.iconst(1);
           let a = block.iconst(5);
           let b = block.iconst(10);
           let c = block.iconst(0);
           let d = block.add(a, b);
           block.do_if(p)
           	   .ret(&[c])
           	   .ret(&[d])
       });
       let unit = unit.finalize(Box::new([Type::Int32]));
       eprintln!("{}", unit.human_format());
    }
}
