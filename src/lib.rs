mod builder;
mod data;
mod format;
mod util;
mod verification;

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
            let two = block.iconst(Type::Int32, 2);
            let cond = block.less([n, two]);
            block.do_if(cond).ret(&[n]).branch(&b1, &[])
        });
        unit.with_block(b1, |mut block| {
            let one = block.iconst(Type::Int32, 1);
            let a = block.sub([n, one]);
            let fa = block.recurse(&[a]);
            let two = block.iconst(Type::Int32, 2);
            let b = block.sub([n, two]);
            let fb = block.recurse(&[b]);
            let o = block.add([fa, fb]);
            block.ret(&[o])
        });
        let unit = unit.finalize(Type::Int32);
        eprintln!("{}", unit.human_format());
    }
    #[test]
    fn construct() {
        let mut unit = Unit::new();
        let b0 = unit.new_block(&[]);
        unit.with_block(b0, |mut block| {
            let p = block.iconst(Type::Int32, 1);
            let a = block.iconst(Type::Int32, 5);
            let b = block.iconst(Type::Int32, 10);
            let c = block.iconst(Type::Int32, 0);
            let d = block.add([a, b]);
            block.do_if(p).ret(&[c]).ret(&[d])
        });
        let unit = unit.finalize(Type::Int32);
        eprintln!("{}", unit.human_format());
    }
    #[test]
    fn lots_unused() {
        let mut unit = Unit::new();
        unit.settings.volatile = false;
        let b0 = unit.new_block(&[]);
        unit.with_block(b0, |mut block| {
            let a = block.iconst(Type::Int32, 69);
            let _ = block.iconst(Type::Int32, 420);
            let b = block.iconst(Type::Int32, 1);
            let _ = block.add([a, b]);
            block.ret(&[])
        });
        let unit = unit.finalize(Type::Void);
        eprintln!("{}", unit.human_format());
    }
    #[test]
    fn some_unused() {
        let mut unit = Unit::new();
        unit.settings.volatile = false;
        let b0 = unit.new_block(&[]);
        unit.with_block(b0, |mut block| {
            let _ = block.iconst(Type::Int32, 1);
            let a = block.iconst(Type::Int32, 5);
            let b = block.iconst(Type::Int32, 10);
            let _ = block.iconst(Type::Int32, 0);
            let d = block.add([a, b]);
            block.ret(&[d])
        });
        let unit = unit.finalize(Type::Void);
        eprintln!("{}", unit.human_format());
    }
    #[test]
    #[should_panic]
    fn invalid_use() {
        let mut unit = Unit::new();
        let b0 = unit.new_block(&[]);
        let b1 = unit.new_block(&[]);
        let b2 = unit.new_block(&[]);
        let b3 = unit.new_block(&[]);
        let mut n = Default::default();
        unit.with_block(b0, |mut block| {
            let c = block.iconst(Type::Int32, 1);
            block.do_if(c).branch(&b1, &[]).branch(&b2, &[])
        });
        unit.with_block(b1, |mut block| {
            n = block.iconst(Type::Int32, 2);
            block.branch(&b3, &[])
        });
        unit.with_block(b2, |block| block.branch(&b3, &[]));
        unit.with_block(b3, |block| block.ret(&[n]));
        unit.finalize(Type::Int32);
    }
}
