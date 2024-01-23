mod unit;

#[cfg(test)]
mod tests {
    use super::unit;
    #[test]
    fn test() {
		let mut unit = unit::Unit::new();
		let b = unit.new_block(Vec::new());
    }
    //#[test]
    //fn fib() {
	//	let mut unit = ir::Unit::new(&[ir::Type::Int32]);
	//	let b0 = unit.entry_block();
	//	let b1 = unit.new_block(&[]);
	//	let n;
	//	{
    //		let mut b0 = unit.switch_to(&b0);
	//		n = b0.push().fetch_arg(0);
	//		let two = b0.push().iconst(2);
	//		let cond = b0.push().less(n, two);
	//		b0.terminate().branch_if(cond,
    //			[ ir::ret(&[n])
    //    		, ((&b1).into(), &[])]);
	//	}
	//	{
    //		let mut b1 = unit.switch_to(&b1);
    //    	let one = b1.push().iconst(1);
	//		let a = b1.push().sub(n, one);
	//		let fa = b1.push().recurse(&[a]);
	//		let two = b1.push().iconst(2);
	//		let b = b1.push().sub(n, two);
	//		let fb = b1.push().recurse(&[b]);
	//		let o = b1.push().add(fa, fb);
	//		b1.terminate().branch(ir::ret(&[o]));
	//	}
	//	let unit = unit.finalize(&[ir::Type::Int32]);
    //    eprintln!("{}", unit.human_format());
    //}
    //#[test]
    //fn construct() {
    //    let mut unit = ir::Unit::new(&[]);
    //    let b0 = unit.entry_block();
    //    let mut b0 = unit.switch_to(&b0);
    //    let p = b0.push().iconst(1);
    //    let a = b0.push().iconst(5);
    //    let b = b0.push().iconst(10);
    //    let c = b0.push().iconst(0);
    //    let d = b0.push().add(a, b);
    //    b0.terminate().branch_if(p,
    //    	[ ir::ret(&[c])
    //    	, ir::ret(&[d])]);
    //    let unit = unit.finalize(&[ir::Type::Int32]);
    //    eprintln!("{}", unit.human_format());
    //}
}
