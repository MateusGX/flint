use std::sync::Arc;

use super::Value;

// ---------------------------------------------------------------------------
// Adding a new instruction
//
// Adding an opcode touches exactly three places, in this order:
//   1. Add a variant to `Instr` below (group it with its siblings — data
//      movement, arithmetic, control flow, ...).
//   2. Add a one-line match arm to the dispatch loop in `vm::Vm::execute`
//      that delegates to its action's file under `vm::ops` — an existing one
//      if this is a new encoding of an action that's already there (like
//      `AddImm` alongside `Add`), or a new file (and `mod` declaration in
//      `vm::ops`) if it's a genuinely new action. See `vm::ops`'s doc comment
//      for the file-per-action convention and how it keeps the dispatch loop
//      itself branch-free.
//   3. Teach the compiler (`crate::lang::compiler`) to emit it — usually a new
//      branch in `compile_instruction` or one of its helpers.
//
// If the new behavior doesn't need a dedicated opcode (e.g. it's a pure
// function over values), prefer adding a *native* instead — see
// `crate::stdlib`'s `mod.rs` for that extension path, which doesn't require
// touching the VM at all.
// ---------------------------------------------------------------------------

/// Index of a general-purpose register (0..NUM_REGISTERS).
pub type Reg = u8;

/// Number of general-purpose registers (`r0`..`r15`).
pub const NUM_REGISTERS: usize = 16;

/// Number of addressable linear-memory slots reachable via `load`/`store`.
pub const MEMORY_SIZE: usize = 4096;

/// A single bytecode instruction with all labels and constants already
/// resolved to concrete indices. This is the format the VM executes directly.
#[derive(Debug, Clone)]
pub enum Instr {
    /// `dst = value`
    LoadInt(Reg, i64),
    /// `dst = float_value`
    LoadFloat(Reg, f64),
    /// `dst = strings[idx]`
    LoadStr(Reg, u32),
    /// `dst = src`
    Mov(Reg, Reg),
    Add(Reg, Reg, Reg),
    Sub(Reg, Reg, Reg),
    Mul(Reg, Reg, Reg),
    Div(Reg, Reg, Reg),
    Mod(Reg, Reg, Reg),
    /// Register-immediate forms, e.g. `add r0, r0, 1` — selected by the
    /// compiler when the right-hand operand is a literal, mirroring how real
    /// assemblers pick an opcode encoding based on operand shape. Keeping
    /// these as distinct opcodes (rather than a tagged operand decoded at
    /// runtime) keeps the dispatch loop branch-free per operand.
    AddImm(Reg, Reg, i64),
    SubImm(Reg, Reg, i64),
    MulImm(Reg, Reg, i64),
    DivImm(Reg, Reg, i64),
    ModImm(Reg, Reg, i64),
    /// Float arithmetic — operands are auto-promoted from Int if needed.
    AddF(Reg, Reg, Reg),
    SubF(Reg, Reg, Reg),
    MulF(Reg, Reg, Reg),
    DivF(Reg, Reg, Reg),
    /// Bitwise integer operations.
    And(Reg, Reg, Reg),
    AndImm(Reg, Reg, i64),
    Or(Reg, Reg, Reg),
    OrImm(Reg, Reg, i64),
    Xor(Reg, Reg, Reg),
    XorImm(Reg, Reg, i64),
    Not(Reg, Reg),
    Shl(Reg, Reg, Reg),
    ShlImm(Reg, Reg, i64),
    Shr(Reg, Reg, Reg),
    ShrImm(Reg, Reg, i64),
    /// Arithmetic negation: works on `Int` and `Float`.
    Neg(Reg, Reg),
    /// Stores the type name of `src` as a `str` in `dst`.
    TypeOf(Reg, Reg),
    /// Compares two integer registers and updates the flags used by jumps.
    Cmp(Reg, Reg),
    /// Compares a register against an integer literal.
    CmpImm(Reg, i64),
    Jmp(usize),
    /// Jump if the last `cmp` found the operands equal.
    Je(usize),
    Jne(usize),
    Jl(usize),
    Jg(usize),
    Jle(usize),
    Jge(usize),
    Push(Reg),
    Pop(Reg),
    /// Pushes the return address and jumps to the target instruction.
    Call(usize),
    /// Pops a return address and jumps back to it.
    Ret,
    /// `dst = memory[addr]`
    Load(Reg, Reg),
    /// `memory[addr] = src`
    Store(Reg, Reg),
    /// Calls into a Rust-implemented native function by name (looked up in
    /// the constant pool), passing register values as arguments and
    /// optionally writing the result into `dst`.
    NCall {
        name_idx: u32,
        args: Vec<Reg>,
        dst: Option<Reg>,
    },
    Hlt,
}

/// A compiled program: resolved instructions plus the string constant pool
/// that `LoadStr` and `NCall` reference by index.
#[derive(Debug, Clone, Default)]
pub struct Program {
    pub instructions: Vec<Instr>,
    pub strings: Vec<Arc<str>>,
    /// Initial contents of linear memory, indexed by address — populated
    /// from `section .data`/`.bss` declarations and seeded into `Vm::memory`
    /// on first `call`.
    pub initial_memory: Vec<Value>,
}
