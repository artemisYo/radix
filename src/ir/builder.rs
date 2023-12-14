use super::*;

pub fn ret(value: &[InstIdx]) -> (BlockIdx, &[InstIdx]) {
	((u16::MAX as usize).into(), value)
}
