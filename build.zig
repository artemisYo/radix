const std = @import("std");

pub fn build(b: *std.Build) void {
    // no lib build or install step as it should be
    // simply imported, or if using ffi or other
    // weird setups, modification of the build step
    // will be needed anyway
    // tests:
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const main_tests = b.addTest(.{
        .root_source_file = .{ .path = "src/test.zig" },
        .target = target,
        .optimize = optimize,
    });

    const run_main_tests = b.addRunArtifact(main_tests);
    const test_step = b.step("test", "Run library tests");
    test_step.dependOn(&run_main_tests.step);
}
