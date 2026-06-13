/// An operand as written in source, before label/constant resolution.
#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    /// `r0`..`r15`
    Reg(u8),
    /// A bare integer literal, e.g. `10` or `-1`.
    Imm(i64),
    /// A float literal, e.g. `3.14` or `-0.5`.
    Float(f64),
    /// A quoted string literal.
    Str(String),
    /// `[r3]` — a memory address held in a register, used by `load`/`store`.
    Mem(u8),
    /// Any other bareword: a label reference (`jmp loop`) or a native
    /// function name (`ncall debug.print, r0`). Which it is depends on the
    /// instruction it appears in — resolved during compilation. A name
    /// starting with `.` (e.g. `jmp .found`) refers to a label local to the
    /// nearest preceding global label.
    Ident(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    /// Lower-cased mnemonic, e.g. `"mov"`.
    pub mnemonic: String,
    pub operands: Vec<Operand>,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    /// `name:` — a global label, or `.name:` — a label local to the nearest
    /// preceding global label. Both are callable via `call`/`jmp`; only
    /// global labels can be targeted by `route` directives. Inside `section
    /// .data`/`.bss`, a label instead names a memory cell (its address),
    /// declared by the `data`/`res` pseudo-instruction that follows it.
    Label {
        name: String,
        line: usize,
    },
    /// `route METHOD "/path" -> handler` — a declarative directive mapping
    /// an HTTP method and path to a label. Carries no bytecode of its own;
    /// the compiler turns it into routing metadata alongside the program.
    Route {
        method: String,
        path: String,
        handler: String,
        line: usize,
    },
    /// `section .text` | `section .data` | `section .bss` — switches which
    /// region subsequent labels/instructions belong to. Carries no bytecode
    /// of its own.
    Section {
        /// `".text"`, `".data"`, or `".bss"`.
        name: String,
        line: usize,
    },
    Instruction(Instruction),
}

/// A parsed program: an ordered sequence of labels and instructions, with
/// labels not yet resolved to addresses.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Program {
    pub items: Vec<Item>,
}
