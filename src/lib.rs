mod utils;
mod ir;

#[cfg(test)]
mod tests {
    use super::ir;
    #[test]
    fn fib() {
		let mut unit = ir::Builder::new(&[ir::Type::Int32]);
		let n = unit.push().fetch_arg(0);
		let two = unit.push().iconst(2);
		let cond = unit.push().less(n, two);
		unit.push().branch_if(cond,
    	[ ir::ret(&[n])
        , (ir::BlockIdx::from(1), &[])]);
        unit.new_block(&[]);
        let one = unit.push().iconst(1);
		let a = unit.push().sub(n, one);
		let fa = unit.push().recurse(&[a]);
		let two = unit.push().iconst(2);
		let b = unit.push().sub(n, two);
		let fb = unit.push().recurse(&[b]);
		let o = unit.push().add(fa, fb);
		unit.push().branch(ir::ret(&[o]));
		let unit = unit.finalize(&[ir::Type::Int32]);
        eprintln!("{}", unit.human_format());
    }
    #[test]
    fn construct() {
        let mut unit = ir::Builder::new(&[]);
        let p = unit.push().iconst(1);
        let a = unit.push().iconst(5);
        let b = unit.push().iconst(10);
        let c = unit.push().iconst(0);
        let d = unit.push().add(a, b);
        unit.push().branch_if(p,
        [ ir::ret(&[c])
        , ir::ret(&[d])]);
        let unit = unit.finalize(&[ir::Type::Int32]);
        eprintln!("{}", unit.human_format());
    }
}
