const std = @import("std");
const utils = @import("utils.zig");
const com = @import("commons.zig");

pub const Builder  = struct {
    const Blocks = utils.AutoKeyVec(com.BlockIdx, com.Block);
    const Insts = utils.AutoKeyVec(com.InstIdx, com.Instruction);
    const Args = utils.AutoKeyVec(com.InstIdx, com.InstArg);
    const Sigs = utils.AutoRangeVec(com.SigIdx, com.Type);
    const ExArgs = utils.AutoRangeVec(com.ArgIdx, u32);

    allocator: std.mem.Allocator,
    // inits with default constructed b0
    blocks: Blocks,
    // first sig is function sig
    signatures: Sigs,
    instructions: Insts,
    arguments: Args,
    extra_args: ExArgs,
    // appended to signatures upon finalize
    return_sig: []com.Type,

    const Self = @This();
    pub fn init(
        signature: []com.Type,
        return_signature: []com.Type,
        alloc: std.mem.Allocator
    ) std.mem.Allocator.Error!Self {
        var out = .{
            .allocator = alloc,
            .blocks = try Blocks.init(alloc),
            .instructions = try Insts.init(alloc),
            .arguments = try Args.init(alloc),
            .signatures = try Sigs.init(alloc),
            .extra_args = try ExArgs.init(alloc),
            .return_sig = return_signature,
        };
        const sig = try out.signatures.push(alloc, signature);
        try out.blocks.push(alloc, .{
            .sig = sig,
            .start = com.InstIdx.from(0),
            .terminator = com.InstIdx.from(0),
        });
        return out;
    }
    pub fn deinit(self: Self) void {
        self.blocks.deinit(self.allocator);
        self.instructions.deinit(self.allocator);
        self.arguments.deinit(self.allocator);
        self.signatures.deinit(self.allocator);
        self.extra_args.deinit(self.allocator);
    }
    pub fn finish(self: *Self) std.mem.Allocator.Error!@import("finals.zig").Unit {
        self.signatures.push(self.allocator, self.return_sig);
        self.allocator.free(self.return_sig);
        return .{
            .blocks = try self.blocks.to_owned_slice(self.allocator),
            .instructions = try self.instructions.to_owned_slice(self.allocator),
            .arguments = try self.arguments.to_owned_slice(self.allocator),
            .signatures = try self.signatures.to_owned_slice(self.allocator),
            .extra_args = try self.extra_args.to_owned_slice(self.allocator),
        };
    }
};

