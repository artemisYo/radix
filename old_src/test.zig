const std = @import("std");
const impl = @import("main.zig");
const utils = @import("utils.zig");

test "make code alive" {
    var Alloc = std.heap.GeneralPurposeAllocator(.{}){};
    var allocator = Alloc.allocator();
    const KeyType = utils.MakeKey(0);
    const ValueType = struct {};
    const KVMap = utils.AutoKeyVec(KeyType, ValueType);
    var kvmap = try KVMap.init(allocator);
    const idx = try kvmap.push(allocator, .{});
    _ = kvmap.index(idx);
}

test "check all" {
    std.testing.refAllDeclsRecursive(impl);
}