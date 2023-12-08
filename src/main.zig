const std = @import("std");
const utils = @import("utils.zig");
const com = @import("commons.zig");

const SigStore = utils.AutoRangeVec(com.SigIdx, com.Type);
const ArgStore = utils.AutoKeyVec(com.ArgIdx, u32);

const Builder  = struct {
    allocator: std.mem.Allocator,
    // v the first block is always present as an entry block
    blocks: utils.AutoKeyVec(com.BlockIdx, com.Block),
    instructions: utils.AutoKeyVec(com.InstIdx, com.Instruction),
    arguments: utils.AutoKeyVec(com.InstIdx, com.InstArg),
    signatures: SigStore,
    extra_args: ArgStore,
};

