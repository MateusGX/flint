//! `div` — integer division. `Div`/`DivImm` are one action with two operand
//! encodings (see [`super::add`] for why arithmetic ops come in pairs like
//! this), but unlike `add`/`sub`/`mul` it needs a division-by-zero check
//! before the checked operation, so it doesn't go through `binary_int`.

use crate::vm::error::VmError;
use crate::vm::instr::Reg;
use crate::vm::value::Value;
use crate::vm::Vm;

fn checked_div(vm: &mut Vm, dst: Reg, a: Reg, rhs: i64, pc: usize) -> Result<(), VmError> {
    let lhs = vm.int(a, pc)?;
    if rhs == 0 {
        return Err(VmError::new(pc, "division by zero"));
    }
    let result = lhs
        .checked_div(rhs)
        .ok_or_else(|| VmError::new(pc, "integer overflow in 'div'"))?;
    vm.set(dst, Value::Int(result));
    Ok(())
}

pub(crate) fn exec(vm: &mut Vm, dst: Reg, a: Reg, b: Reg, pc: usize) -> Result<(), VmError> {
    let rhs = vm.int(b, pc)?;
    checked_div(vm, dst, a, rhs, pc)
}

pub(crate) fn exec_imm(vm: &mut Vm, dst: Reg, a: Reg, imm: i64, pc: usize) -> Result<(), VmError> {
    checked_div(vm, dst, a, imm, pc)
}
