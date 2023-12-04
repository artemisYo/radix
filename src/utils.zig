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

pub fn AutoKeyMap(comptime Key: type, comptime Value: type) type {
    // check for correct methods being there
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
        pub fn push(self: *Self, allocator: std.mem.Allocator, value: Value) std.mem.Allocator.Error!Key {
            const out = self.new_key();
            try self.entries.append(allocator, value);
            return out;
        }
        pub fn index(self: *const Self, key: Key) *Value {
            return &self.entries.items[key.into()];
        }
    };
}

