const utils = @import("utils.zig");
const com = @import("commons.zig");

pub const Unit = struct {
	blocks: utils.AutoKeySlice(com.BlockIdx, com.Block),
	instructions: utils.AutoKeySlice(com.InstIdx, com.Instruction),
	arguments: utils.AutoKeySlice(com.InstIdx, com.InstArg),
	signatures: utils.AutoRangeSlice(com.SigIdx, com.Type),
	extra_args: utils.AutoRangeSlice(com.ArgIdx, u32),
};
