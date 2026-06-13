//! `mod` — integer remainder. `Mod`/`ModImm` are one action with two operand
//! encodings, and like [`super::div`] needs a division-by-zero check before
//! the checked operation. Named `modulo` — `mod` is a keyword.

use crate::vm::error::VmError;
use crate::vm::instr::Reg;
use crate::vm::value::Value;
use crate::vm::Vm;

fn checked_mod(vm: &mut Vm, dst: Reg, a: Reg, rhs: i64, pc: usize) -> Result<(), VmError> {
    let lhs = vm.int(a, pc)?;
    if rhs == 0 {
        return Err(VmError::new(pc, "division by zero"));
    }
    let result = lhs
        .checked_rem(rhs)
        .ok_or_else(|| VmError::new(pc, "integer overflow in 'mod'"))?;
    vm.set(dst, Value::Int(result));
    Ok(())
}

pub(crate) fn exec(vm: &mut Vm, dst: Reg, a: Reg, b: Reg, pc: usize) -> Result<(), VmError> {
    let rhs = vm.int(b, pc)?;
    checked_mod(vm, dst, a, rhs, pc)
}

pub(crate) fn exec_imm(vm: &mut Vm, dst: Reg, a: Reg, imm: i64, pc: usize) -> Result<(), VmError> {
    checked_mod(vm, dst, a, imm, pc)
}
