//! `sub` — checked integer subtraction. `Sub`/`SubImm` are one action with
//! two operand encodings — see [`super::add`] for why they share a helper.

use crate::vm::error::VmError;
use crate::vm::instr::Reg;
use crate::vm::Vm;

pub(crate) fn exec(vm: &mut Vm, dst: Reg, a: Reg, b: Reg, pc: usize) -> Result<(), VmError> {
    let rhs = vm.int(b, pc)?;
    super::binary_int(vm, dst, a, rhs, pc, "sub", i64::checked_sub)
}

pub(crate) fn exec_imm(vm: &mut Vm, dst: Reg, a: Reg, imm: i64, pc: usize) -> Result<(), VmError> {
    super::binary_int(vm, dst, a, imm, pc, "sub", i64::checked_sub)
}
