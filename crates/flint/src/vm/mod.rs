//! Register-based virtual machine that executes Flint bytecode.
//!
//! This module owns the bytecode format ([`Instr`]/[`Program`]), the runtime
//! value representation ([`Value`]), the native-function bridge
//! ([`NativeRegistry`]) and the interpreter loop ([`Vm`]) — [`crate::lang`]
//! compiles source text down to [`Program`] and depends on this module to do
//! so, since the bytecode format is the contract between the two.
//!
//! `error`/`instr`/`native`/`value` hold the supporting types; `Vm` and its
//! dispatch loop live directly here, with each instruction's behavior split
//! out under [`ops`] — one file per action, so e.g. understanding or fixing
//! `add` means opening `ops/add.rs` and nothing else.

use std::sync::Arc;

mod error;
mod instr;
mod native;
pub(crate) mod ops;
mod value;

pub use error::VmError;
pub use instr::{Instr, Program, Reg, MEMORY_SIZE, NUM_REGISTERS};
pub use native::{NativeFn, NativeRegistry};
pub use value::Value;

pub(crate) use ops::cmp::Flags;

/// Register-based virtual machine that executes a compiled [`Program`].
///
/// State (registers, stack, linear memory, flags) lives on the `Vm` so it can
/// be reused across runs and inspected afterwards (handy for tests and for
/// the future HTTP bridge, which needs to read the result a handler left in
/// its registers).
const MAX_CALL_DEPTH: usize = 1024;
pub const DEFAULT_INSTRUCTION_LIMIT: usize = 1_000_000;

pub struct Vm {
    registers: [Value; NUM_REGISTERS],
    stack: Vec<Value>,
    /// Linear memory, allocated lazily: `memory.len()` grows on demand as
    /// `store` touches higher addresses (capped at `MEMORY_SIZE`), and reads
    /// past the current length yield `Int(0)` — observably identical to a
    /// fully pre-allocated zeroed buffer. This keeps `Vm::new` cheap, which
    /// matters because the HTTP server builds a fresh `Vm` per request.
    memory: Vec<Value>,
    flags: Flags,
    pc: usize,
    call_depth: usize,
    natives: NativeRegistry,
}

impl Vm {
    pub fn new(natives: NativeRegistry) -> Self {
        Self {
            registers: std::array::from_fn(|_| Value::Int(0)),
            stack: Vec::new(),
            memory: Vec::new(),
            flags: Flags::default(),
            pc: 0,
            call_depth: 0,
            natives,
        }
    }

    /// Reads the current value of a register. Useful for inspecting the
    /// result a program left behind once `run` returns.
    pub fn register(&self, reg: Reg) -> Option<&Value> {
        self.registers.get(reg as usize)
    }

    /// Executes `program` from its first instruction until it halts (via
    /// `hlt` or by running off the end of the instruction list).
    pub fn run(&mut self, program: &Program) -> Result<(), VmError> {
        self.run_with_instruction_limit(program, DEFAULT_INSTRUCTION_LIMIT)
    }

    pub fn run_with_instruction_limit(
        &mut self,
        program: &Program,
        instruction_limit: usize,
    ) -> Result<(), VmError> {
        self.seed_memory(program);
        self.call_depth = 0;
        self.pc = 0;
        self.execute(program, instruction_limit)
    }

    /// Executes `program` starting at `address`, as if it were called from
    /// outside (e.g. an HTTP route handler). Pushes a return address equal to
    /// `program.instructions.len()` — when the callee eventually `ret`s back
    /// to it, `pc` lands one past the end of the program, and the dispatch
    /// loop's existing "ran off the end" check halts execution. No special
    /// sentinel value or extra halt opcode needed.
    pub fn call(&mut self, program: &Program, address: usize) -> Result<(), VmError> {
        self.call_with_instruction_limit(program, address, DEFAULT_INSTRUCTION_LIMIT)
    }

    pub fn call_with_instruction_limit(
        &mut self,
        program: &Program,
        address: usize,
        instruction_limit: usize,
    ) -> Result<(), VmError> {
        if address >= program.instructions.len() {
            return Err(VmError::new(
                address,
                format!("call address {address} is outside the program"),
            ));
        }
        self.seed_memory(program);
        self.call_depth = 0;
        self.stack
            .push(Value::Int(program.instructions.len() as i64));
        self.pc = address;
        self.execute(program, instruction_limit)
    }

    /// Seeds linear memory from `program.initial_memory` (the `section
    /// .data`/`.bss` image) the first time this `Vm` runs anything. A no-op
    /// once `memory` is non-empty, so it never clobbers state from an earlier
    /// `run`/`call` on the same `Vm`.
    fn seed_memory(&mut self, program: &Program) {
        if self.memory.is_empty() && !program.initial_memory.is_empty() {
            self.memory = program.initial_memory.clone();
        }
    }

    /// The dispatch loop. Each instruction's *behavior* lives in its own
    /// file under `vm::ops` (one per action — see that module's doc comment
    /// for why); the `match` here stays a flat, centralized jump table over
    /// the `Instr` discriminant — Rust enums and `match` arms can't be split
    /// across files, and this is also the "branch-free" shape `instr.rs`
    /// documents as a deliberate performance choice. Delegating to a plain
    /// function per arm doesn't change that: it's a direct, inlinable call,
    /// not a vtable dispatch.
    fn execute(&mut self, program: &Program, instruction_limit: usize) -> Result<(), VmError> {
        let mut executed = 0usize;
        loop {
            let Some(instr) = program.instructions.get(self.pc) else {
                return Ok(());
            };
            let pc = self.pc;
            if executed >= instruction_limit {
                return Err(VmError::new(
                    pc,
                    format!("instruction limit exceeded ({instruction_limit})"),
                ));
            }
            executed += 1;
            let mut next_pc = pc + 1;

            match instr {
                // `hlt` ends execution right here — there's no state to
                // update and nowhere else the "return" can live, so unlike
                // every other instruction it has no `ops` file of its own.
                Instr::Hlt => return Ok(()),

                Instr::LoadInt(dst, value) => ops::mov::exec_load_int(self, *dst, *value),
                Instr::LoadFloat(dst, value) => ops::float::exec_load_float(self, *dst, *value),
                Instr::LoadStr(dst, idx) => ops::mov::exec_load_str(self, program, *dst, *idx, pc)?,
                Instr::Mov(dst, src) => ops::mov::exec_mov(self, *dst, *src),

                Instr::Add(dst, a, b) => ops::add::exec(self, *dst, *a, *b, pc)?,
                Instr::AddImm(dst, a, imm) => ops::add::exec_imm(self, *dst, *a, *imm, pc)?,
                Instr::Sub(dst, a, b) => ops::sub::exec(self, *dst, *a, *b, pc)?,
                Instr::SubImm(dst, a, imm) => ops::sub::exec_imm(self, *dst, *a, *imm, pc)?,
                Instr::Mul(dst, a, b) => ops::mul::exec(self, *dst, *a, *b, pc)?,
                Instr::MulImm(dst, a, imm) => ops::mul::exec_imm(self, *dst, *a, *imm, pc)?,
                Instr::Div(dst, a, b) => ops::div::exec(self, *dst, *a, *b, pc)?,
                Instr::DivImm(dst, a, imm) => ops::div::exec_imm(self, *dst, *a, *imm, pc)?,
                Instr::Mod(dst, a, b) => ops::modulo::exec(self, *dst, *a, *b, pc)?,
                Instr::ModImm(dst, a, imm) => ops::modulo::exec_imm(self, *dst, *a, *imm, pc)?,

                Instr::AddF(dst, a, b) => ops::float::exec_addf(self, *dst, *a, *b, pc)?,
                Instr::SubF(dst, a, b) => ops::float::exec_subf(self, *dst, *a, *b, pc)?,
                Instr::MulF(dst, a, b) => ops::float::exec_mulf(self, *dst, *a, *b, pc)?,
                Instr::DivF(dst, a, b) => ops::float::exec_divf(self, *dst, *a, *b, pc)?,

                Instr::And(dst, a, b) => ops::bitwise::exec_and(self, *dst, *a, *b, pc)?,
                Instr::AndImm(dst, a, imm) => ops::bitwise::exec_and_imm(self, *dst, *a, *imm, pc)?,
                Instr::Or(dst, a, b) => ops::bitwise::exec_or(self, *dst, *a, *b, pc)?,
                Instr::OrImm(dst, a, imm) => ops::bitwise::exec_or_imm(self, *dst, *a, *imm, pc)?,
                Instr::Xor(dst, a, b) => ops::bitwise::exec_xor(self, *dst, *a, *b, pc)?,
                Instr::XorImm(dst, a, imm) => ops::bitwise::exec_xor_imm(self, *dst, *a, *imm, pc)?,
                Instr::Not(dst, src) => ops::bitwise::exec_not(self, *dst, *src, pc)?,
                Instr::Shl(dst, a, b) => ops::bitwise::exec_shl(self, *dst, *a, *b, pc)?,
                Instr::ShlImm(dst, a, imm) => ops::bitwise::exec_shl_imm(self, *dst, *a, *imm, pc)?,
                Instr::Shr(dst, a, b) => ops::bitwise::exec_shr(self, *dst, *a, *b, pc)?,
                Instr::ShrImm(dst, a, imm) => ops::bitwise::exec_shr_imm(self, *dst, *a, *imm, pc)?,

                Instr::Neg(dst, src) => ops::misc::exec_neg(self, *dst, *src, pc)?,
                Instr::TypeOf(dst, src) => ops::misc::exec_typeof(self, *dst, *src),

                Instr::Cmp(a, b) => ops::cmp::exec(self, *a, *b, pc)?,
                Instr::CmpImm(a, imm) => ops::cmp::exec_imm(self, *a, *imm, pc)?,
                Instr::Jmp(target) => next_pc = ops::jump::jmp(*target),
                Instr::Je(target) => next_pc = ops::jump::je(self, *target, next_pc),
                Instr::Jne(target) => next_pc = ops::jump::jne(self, *target, next_pc),
                Instr::Jl(target) => next_pc = ops::jump::jl(self, *target, next_pc),
                Instr::Jg(target) => next_pc = ops::jump::jg(self, *target, next_pc),
                Instr::Jle(target) => next_pc = ops::jump::jle(self, *target, next_pc),
                Instr::Jge(target) => next_pc = ops::jump::jge(self, *target, next_pc),

                Instr::Push(src) => ops::stack::exec_push(self, *src),
                Instr::Pop(dst) => ops::stack::exec_pop(self, *dst, pc)?,
                Instr::Call(target) => next_pc = ops::call::exec_call(self, *target, next_pc, pc)?,
                Instr::Ret => next_pc = ops::call::exec_ret(self, pc)?,

                Instr::Load(dst, addr_reg) => ops::memory::exec_load(self, *dst, *addr_reg, pc)?,
                Instr::Store(addr_reg, src_reg) => {
                    ops::memory::exec_store(self, *addr_reg, *src_reg, pc)?
                }

                Instr::NCall {
                    name_idx,
                    args,
                    dst,
                } => ops::ncall::exec(self, program, *name_idx, args, *dst, pc)?,
            }

            self.pc = next_pc;
        }
    }

    // -----------------------------------------------------------------
    // Internal primitives. `pub(crate)` rather than private: every
    // `vm::ops::*` module is the actual home of an instruction's behavior
    // and orchestrates these to implement it, but they stay invisible
    // outside this crate — `Vm`'s public surface is only what's above.
    // -----------------------------------------------------------------

    pub(crate) fn get(&self, reg: Reg) -> &Value {
        &self.registers[reg as usize]
    }

    pub(crate) fn set(&mut self, reg: Reg, value: Value) {
        self.registers[reg as usize] = value;
    }

    pub(crate) fn int(&self, reg: Reg, pc: usize) -> Result<i64, VmError> {
        self.get(reg).as_int().ok_or_else(|| {
            VmError::new(
                pc,
                format!(
                    "expected an integer in r{reg}, found a value of type '{}'",
                    self.get(reg).type_name()
                ),
            )
        })
    }

    pub(crate) fn string_constant(
        &self,
        program: &Program,
        idx: u32,
        pc: usize,
    ) -> Result<Arc<str>, VmError> {
        program
            .strings
            .get(idx as usize)
            .cloned()
            .ok_or_else(|| VmError::new(pc, format!("invalid string constant index {idx}")))
    }

    pub(crate) fn flags(&self) -> Flags {
        self.flags
    }

    pub(crate) fn set_flags(&mut self, flags: Flags) {
        self.flags = flags;
    }

    pub(crate) fn push_stack(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub(crate) fn pop_stack(&mut self) -> Option<Value> {
        self.stack.pop()
    }

    pub(crate) fn natives(&self) -> &NativeRegistry {
        &self.natives
    }

    pub(crate) fn inc_call_depth(&mut self, pc: usize) -> Result<(), VmError> {
        if self.call_depth >= MAX_CALL_DEPTH {
            Err(VmError::new(
                pc,
                format!("call stack depth exceeded {MAX_CALL_DEPTH}"),
            ))
        } else {
            self.call_depth += 1;
            Ok(())
        }
    }

    pub(crate) fn dec_call_depth(&mut self) {
        self.call_depth = self.call_depth.saturating_sub(1);
    }

    /// Reads `memory[addr]`. Addresses within `MEMORY_SIZE` but beyond the
    /// backing vector's current length haven't been written yet and read as
    /// `Int(0)`, matching the behavior of a fully pre-allocated buffer.
    pub(crate) fn memory_at(&self, addr: i64, pc: usize) -> Result<Value, VmError> {
        let addr = self.check_address(addr, pc)?;
        Ok(self.memory.get(addr).cloned().unwrap_or(Value::Int(0)))
    }

    /// Writes `memory[addr] = value`, growing the backing vector with zeroed
    /// slots as needed (capped at `MEMORY_SIZE`).
    pub(crate) fn memory_store(
        &mut self,
        addr: i64,
        value: Value,
        pc: usize,
    ) -> Result<(), VmError> {
        let addr = self.check_address(addr, pc)?;
        if addr >= self.memory.len() {
            self.memory.resize(addr + 1, Value::Int(0));
        }
        self.memory[addr] = value;
        Ok(())
    }

    fn check_address(&self, addr: i64, pc: usize) -> Result<usize, VmError> {
        if addr < 0 {
            return Err(VmError::new(
                pc,
                format!("memory address {addr} is negative"),
            ));
        }
        let addr = addr as usize;
        if addr >= MEMORY_SIZE {
            return Err(VmError::new(
                pc,
                format!("memory address {addr} is out of bounds (size {MEMORY_SIZE})"),
            ));
        }
        Ok(addr)
    }
}
