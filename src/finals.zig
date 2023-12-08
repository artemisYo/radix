// 80B
const Unit = struct {
	blocks: []Block,
	instructions: []Instruction,
	arguments: []InstArg,
	signatures: []Type,
	extra_args: []u32,
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
