const std = @import("std");

pub fn MakeKey(comptime garbage: usize) type {
    return struct {
        // this is needed in order to circumvent memoization of zig's comptime funcs
        // so as to allow new structs to actually be generated (instead of the same one)
        const garb = garbage;
        idx: u32,
        pub fn from(idx: u32) @This() {
            return .{.idx = idx};
        }
        pub fn into(self: @This()) u32 {
            return self.idx;
        }
    };
}

// `garbage`: same as above
pub fn MakeRange(comptime garbage: usize, comptime Idx: type, comptime Len: type) type {
    return packed struct {
        const garb = garbage;
        idx: Idx,
        len: Len,
        pub fn from(idx: Idx, len: Len) @This() {
            return .{.idx = idx, .len = len};
        }
        pub fn into(self: @This()) struct {Idx, Len} {
            return .{self.idx, self.len};
        }
    };
}

pub fn AutoRangeSlice(comptime Range: type, comptime Value: type) type {
    comptime RangeInterface(Range);
    return struct {
        entries: []Value,

        pub fn index(self: *@This(), range: Range) []Value {
            const idx = range.into();
            return self.entries[@intCast(idx[0])..][0..@intCast(idx[1])];
        }
    };
}

// Used to intern slices into a single flat array, as is similarly done
// for string interning
pub fn AutoRangeVec(comptime Range: type, comptime Value: type) type {
    comptime RangeInterface(Range);
    return struct {
        entries: std.ArrayListUnmanaged(Value),

        const Self = @This();
        pub fn init(allocator: std.mem.Allocator) std.mem.Allocator.Error!Self {
            const entries = try std.ArrayListUnmanaged(Value).initCapacity(allocator, 10);
            return .{.entries = entries};
        }
        pub fn deinit(self: *Self, allocator: std.mem.Allocator) void {
            self.entries.deinit(allocator);
        }
        pub fn new_range(self: *const Self) Range {
            return Range.from(
                @intCast(self.entries.items.len),
                0
            );
        }
        pub fn push(
            self: *Self,
            allocator: std.mem.Allocator,
            value: []Value
        ) std.mem.Allocator.Error!Range {
            var out = self.new_key();
            out.len = @intCast(value.len);
            try self.entries.appendSlice(allocator, value);
            return out;
        }
        pub fn index(self: *const Self, range: Range) []Value {
            const idx = range.into();
            return &self.entries.items[idx[0]..][0..idx[1]];
        }
        pub fn to_owned_slice(
            self: *Self,
            allocator: std.mem.Allocator
        ) std.mem.Allocator.Error!AutoKeySlice(Range, Value) {
            return .{ .entries = try self.entries.toOwnedSlice(allocator) };
        }
    };
}

pub fn AutoKeySlice(comptime Key: type, comptime Value: type) type {
    comptime KeyInterface(Key);
    return struct {
        entries: []Value,

        pub fn index(self: *@This(), key: Key) *Value {
            return self.entries[@intCast(key.into())];
        }
    };
}

// AutoKeyMap is a vector that allows the use of
// newtypes to index them, it is unmanaged
// contiguously storing everything in a central place
// allows easy tracking of which instructions exist
// (i.e. blocks do not always start from @0)
// and easier sharing (not having to pass around all
// of the info when not needed)
pub fn AutoKeyVec(comptime Key: type, comptime Value: type) type {
    // check for correct methods being there
    comptime KeyInterface(Key);
    return struct {
        entries: std.ArrayListUnmanaged(Value),

        const Self = @This();
        pub fn init(allocator: std.mem.Allocator) std.mem.Allocator.Error!Self {
            const entries = try std.ArrayListUnmanaged(Value).initCapacity(allocator, 10);
            return .{.entries = entries};
        }
        pub fn deinit(self: *Self, allocator: std.mem.Allocator) void {
            self.entries.deinit(allocator);
        }
        pub fn new_key(self: *const Self) Key {
            const idx: u32 = @intCast(self.entries.items.len);
            return Key.from(idx);
        }
        pub fn push(
            self: *Self,
            allocator: std.mem.Allocator,
            value: Value
        ) std.mem.Allocator.Error!Key {
            const out = self.new_key();
            try self.entries.append(allocator, value);
            return out;
        }
        pub fn index(self: *const Self, key: Key) *Value {
            return &self.entries.items[key.into()];
        }
        pub fn to_owned_slice(
            self: *Self,
            allocator: std.mem.Allocator
        ) std.mem.Allocator.Error!AutoKeySlice(Key, Value) {
            return .{ .entries = try self.entries.toOwnedSlice(allocator) };
        }
    };
}

fn RangeInterface(comptime Range: type) void {
    comptime {
        if (!@hasDecl(Range, "from")) {
            @compileError(
                "Expected a `from` function implemented on range type "
                ++ @typeName(Range)
                ++ "!"
                ++ "\n"
                ++ "The method should accept two numbers to construct the key!"
                ++ "\n"
                ++ "If you have implemented it, ensure the function is public!");
        }
        if (!@hasDecl(Range, "into")) {
            @compileError(
                "Expected an `into` function implemented on range type "
                ++ @typeName(Range)
                ++ "!"
                ++ "\n"
                ++ "The method should accept the key by value and return the index and the length!"
                ++ "\n"
                ++ "If you have implemented it, ensure the function is public!");
        }
    }
}

fn KeyInterface(comptime Key: type) void {
    comptime {
        if (!@hasDecl(Key, "from")) {
            @compileError(
                "Expected a `from` function implemented on key type "
                ++ @typeName(Key)
                ++ "!"
                ++ "\n"
                ++ "The method should accept a u32 to construct the key!"
                ++ "\n"
                ++ "If you have implemented it, ensure the function is public!");
        }
        if (@TypeOf(Key.from) != fn(u32) Key) {
            @compileError(
                "Expected the `from` function in key type "
                ++ @typeName(Key)
                ++ " to be of type `fn(u32) @This`!");
        }
        if (!@hasDecl(Key, "into")) {
            @compileError(
                "Expected an `into` function implemented on key type "
                ++ @typeName(Key)
                ++ "!"
                ++ "\n"
                ++ "The method should accept the key by value and return a u32!"
                ++ "\n"
                ++ "If you have implemented it, ensure the function is public!");
        }
        if (@TypeOf(Key.into) != fn(Key) u32) {
            @compileError(
                "Expected the `into` function in key type "
                ++ @typeName(Key)
                ++ " to be of type `fn(@This) u32`!");
        }
    }
}

