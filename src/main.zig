const std = @import("std");
const utils = @import("utils.zig");

const InstIdx = utils.MakeKey(0);
const BlockIdx = utils.MakeKey(1);

// TODO: template this
const SigIdx = packed struct {
	idx: u24,
	len: u8,

	const Self = @This();
	pub fn from(index: u24, length: u8) Self {
		return .{.idx = index, .len = length};
	}
};

// stores signatures in a flattened manner
// dishes out the index with a length, basically a smaller
// slice into it
// since not many or long signatures are expected, this uses
// a u24 as the index and a u8 as length
// TODO: make this a template
const SigStore = struct {
	sigs: std.ArrayListUnmanaged(Type),

	const Self = @This();
	pub fn append(
		self: *Self,
		allocator: std.mem.Allocator,
		data: []Type
	) std.mem.Allocator.Error!SigIdx {
    	const idx: u24 = @intCast(self.sigs.items.len);
    	const len: u8 = @intCast(data.len);
		try self.sigs.appendSlice(allocator, data);
		return SigIdx.from(idx, len);
	}
	pub fn index(self: *const Self, idx: SigIdx) []Type {
		return &self.sigs.items[@intCast(idx.idx)..][0..@intCast(idx.len)];
	}
};

// similar to sigstore, in that it stores
// variable length argument lists
const ArgStore = struct {
	sigs: std.ArrayListUnmanaged(u32),

	const Self = @This();
	pub fn append(
		self: *Self,
		allocator: std.mem.Allocator,
		data: []u32
	) std.mem.Allocator.Error!SigIdx {
    	const idx: u24 = @intCast(self.sigs.items.len);
    	const len: u8 = @intCast(data.len);
		try self.sigs.appendSlice(allocator, data);
		return SigIdx.from(idx, len);
	}
	pub fn index(self: *const Self, idx: SigIdx) []Type {
		return &self.sigs.items[@intCast(idx.idx)..][0..@intCast(idx.len)];
	}
};

// size: 136B
const Unit = struct {
    allocator: std.mem.Allocator,
    // AutoKeyMap is a vector that allows the use of
    // newtypes to index them, it is unmanaged
    // contiguously storing everything in a central place
    // allows easy tracking of which instructions exist
    // (i.e. blocks do not always start from @0)
    // and easier sharing (not having to pass around all
    // of the info when not needed)
    // v the first block is always present as an entry block
    blocks: utils.AutoKeyMap(BlockIdx, Block),
    instructions: utils.AutoKeyMap(InstIdx, Instruction),
    arguments: utils.AutoKeyMap(InstIdx, InstArg),
    signatures: SigStore,
    outofband_args: ArgStore,
};

// 16B
const Block = struct {
    sig: SigIdx,
    // the first instruction of the block,
    // size: 4B * 2
    start: InstIdx,
    // the terminator instruction, end of the block
    terminator: InstIdx,
};
// Unused currently, as I am too lazy to reimplement
// an ArrayList just to use the length as a presence
// switch
//const InstQuad = struct {
//	kinds: [4]Instruction,
//	args: [4]InstArg,
//};
const InstArg = union {
    // the tag is the associated instruction kind
    // as instructions always take the same
    // arg shapes
	OutOfBand: u32,
	InBand: u32,
};
const Instruction = enum(u8) {
	IntConst,
	Add,
	Sub,
	Mult,
	Div,
	Branch,
};
const Type = enum {
	i8, i16, i32, i64,
};
