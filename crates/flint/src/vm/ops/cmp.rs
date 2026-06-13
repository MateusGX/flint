//! `cmp` — compares two integers and records the result in [`Flags`], which
//! [`super::jump`] reads to decide whether to branch. `Cmp`/`CmpImm` are one
//! action with two operand encodings, same story as the arithmetic ops.

use crate::vm::error::VmError;
use crate::vm::instr::Reg;
use crate::vm::Vm;

/// Flags set by `cmp`/`cmp_imm`, consumed by the conditional jumps in
/// [`super::jump`]. `compare` is the only thing that ever constructs one,
/// which is why it lives here rather than as a bare field on `Vm`.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Flags {
    pub(crate) eq: bool,
    pub(crate) lt: bool,
    pub(crate) gt: bool,
}

fn compare(vm: &mut Vm, a: Reg, rhs: i64, pc: usize) -> Result<(), VmError> {
    let lhs = vm.int(a, pc)?;
    vm.set_flags(Flags {
        eq: lhs == rhs,
        lt: lhs < rhs,
        gt: lhs > rhs,
    });
    Ok(())
}

pub(crate) fn exec(vm: &mut Vm, a: Reg, b: Reg, pc: usize) -> Result<(), VmError> {
    let rhs = vm.int(b, pc)?;
    compare(vm, a, rhs, pc)
}

pub(crate) fn exec_imm(vm: &mut Vm, a: Reg, imm: i64, pc: usize) -> Result<(), VmError> {
    compare(vm, a, imm, pc)
}
