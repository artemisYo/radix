const utils = @import("utils.zig");

// this file is used to keep declare types used
// across namespaces

pub const InstIdx = utils.MakeKey(0);
pub const BlockIdx = utils.MakeKey(1);
pub const SigIdx = utils.MakeRange(0, u24, u8);
pub const ArgIdx = utils.MakeRange(1, u24, u8);

pub const Block = struct {
    sig: SigIdx,
    // the first instruction of the block,
    // size: 4B * 2
    start: InstIdx,
    // the terminator instruction, end of the block
    terminator: InstIdx,
};

pub const InstArg = union {
    // the tag is the associated instruction kind
    // as instructions always take the same
    // arg shapes
	OutOfBand: u32,
	InBand: u32,
};

pub const Instruction = enum(u8) {
	IntConst,
	Add,
	Sub,
	Mult,
	Div,
	Branch,
};

pub const Type = enum {
	i8, i16, i32, i64,
};
