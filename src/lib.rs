mod utils;
mod ir;

#[cfg(test)]
mod tests {
    use super::ir;
    #[test]
    fn construct() {
        let mut unit = ir::Unit::new(&[]);
        let v = unit.push().iconst(5);
        unit.push().ret(v);
        let unit = unit.finalize(&[ir::Type::Int32]);
        eprintln!("{}", unit.human_format());
    }
    #[test]
    fn it_works() {
        let result = 2usize.pow(2);
        assert_eq!(result, 4);
    }
}
