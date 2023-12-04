const std = @import("std");
// Vec is shorter and I'm lazy
const Vec = std.ArrayList;

const utils = @import("utils.zig");

const InstIdx = utils.MakeKey(0);
const BlockIdx = utils.MakeKey(1);

// 64B + sizeOf(sig)
const Unit = struct {
    // AutoKeyMap is a vector that allows the use of
    // newtypes to index them, it is unmanaged
    // contiguously storing everything in a central place
    // allows easy tracking of which instructions exist
    // (i.e. blocks do not always start from @0)
    // and easier sharing (not having to pass around all
    // of the info when not needed)
    const BlockStore = utils.AutoKeyMap(BlockIdx, Block);
    const InstStore = utils.AutoKeyMap(Instruction);
    allocator: std.mem.Allocator,
    // rest of the blocks in the body
    // technically unordered as the 'order' is simply
    // branching to them (i.e. control-flow-graph)
    blocks: BlockStore,
    instructions: InstStore,
    // the exit body is predetermined (just return the params)
    // is also the direct return type (tuples and so on are not
    // returned, but rather written into the caller's stackframe)
    exit: Signature,
};

// 8B + sizeOf(sig)
const Block = struct {
    sig: Signature,
    // the first instruction of the block,
    // size: 4B * 2
    start: InstIdx,
    // the terminator instruction, end of the block
    terminator: InstIdx,
};
const Signature = struct {
    // parameter types, they are accessed by index
    // i.e. `getArg !0` is just the parameters[0] arg
    parameters: void,
};
const Instruction = enum {};
