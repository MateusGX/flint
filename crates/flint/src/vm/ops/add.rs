//! `add` — checked integer addition. `Add` (register operand) and `AddImm`
//! (immediate operand) are the same action; the compiler picks the encoding
//! based on the right-hand operand's shape, so both funnel through the same
//! `binary_int` helper in [`super`].

use crate::vm::error::VmError;
use crate::vm::instr::Reg;
use crate::vm::Vm;

pub(crate) fn exec(vm: &mut Vm, dst: Reg, a: Reg, b: Reg, pc: usize) -> Result<(), VmError> {
    let rhs = vm.int(b, pc)?;
    super::binary_int(vm, dst, a, rhs, pc, "add", i64::checked_add)
}

pub(crate) fn exec_imm(vm: &mut Vm, dst: Reg, a: Reg, imm: i64, pc: usize) -> Result<(), VmError> {
    super::binary_int(vm, dst, a, imm, pc, "add", i64::checked_add)
}
