//! Execution logic for each VM instruction, one file per *action* — `add`
//! and `add_imm` are the same action with different operand encodings (the
//! compiler picks one based on operand shape, exactly like a real assembler
//! choosing an instruction encoding), so they live together; the seven
//! conditional/unconditional jumps share `jump` for the same reason.
//!
//! This mirrors how the language documents its own instruction set
//! (`docs/reference/instructions.md`) — "the action as the Flint programmer
//! sees it" rather than "the enum variant as the compiler emits it" — so
//! the file you'd open to understand `add` from either side is the same one.
//!
//! The dispatch `match` itself stays in `vm::Vm::execute`: Rust enums and
//! `match` arms over them can't be split across files, and routing each
//! instruction through a trait object would trade the VM's documented
//! branch-free dispatch for vtable calls. What's separated here is the
//! *body* — a plain function taking `&mut Vm`, called directly (and
//! inlinable) from the match arm. Adding an instruction now means: a variant
//! in `instr.rs`, a one-line arm in the match, and a function in its file
//! here (new or existing) — see `instr.rs` for the full extension guide.

pub(super) mod add;
pub(super) mod bitwise;
pub(super) mod call;
pub(super) mod cmp;
pub(super) mod div;
pub(super) mod float;
pub(super) mod jump;
pub(super) mod memory;
pub(super) mod misc;
pub(super) mod modulo;
pub(super) mod mov;
pub(super) mod mul;
pub(super) mod ncall;
pub(super) mod stack;
pub(super) mod sub;

use crate::vm::error::VmError;
use crate::vm::instr::Reg;
use crate::vm::value::Value;

use super::Vm;

/// Applies a checked binary integer operation (`a OP rhs`) and stores the
/// result in `dst`. Shared by `add`/`sub`/`mul` (and their `*_imm` forms),
/// which differ only in which `checked_*` operation they pass. `div`/`mod`
/// don't use this because they need a division-by-zero check before their
/// checked operation. `rhs` is already resolved: the caller decides whether
/// it came from a register or an immediate operand.
pub(super) fn binary_int(
    vm: &mut Vm,
    dst: Reg,
    a: Reg,
    rhs: i64,
    pc: usize,
    op_name: &'static str,
    op: fn(i64, i64) -> Option<i64>,
) -> Result<(), VmError> {
    let lhs = vm.int(a, pc)?;
    let result =
        op(lhs, rhs).ok_or_else(|| VmError::new(pc, format!("integer overflow in '{op_name}'")))?;
    vm.set(dst, Value::Int(result));
    Ok(())
}
