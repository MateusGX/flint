use crate::vm::{error::VmError, instr::Reg, value::Value, Vm};

fn validate_shift(amount: i64, op: &'static str, pc: usize) -> Result<u32, VmError> {
    if !(0..=63).contains(&amount) {
        return Err(VmError::new(
            pc,
            format!("shift amount must be 0–63 in '{op}', found {amount}"),
        ));
    }
    Ok(amount as u32)
}

pub(crate) fn exec_and(vm: &mut Vm, dst: Reg, a: Reg, b: Reg, pc: usize) -> Result<(), VmError> {
    let lhs = vm.int(a, pc)?;
    let rhs = vm.int(b, pc)?;
    vm.set(dst, Value::Int(lhs & rhs));
    Ok(())
}

pub(crate) fn exec_and_imm(
    vm: &mut Vm,
    dst: Reg,
    a: Reg,
    imm: i64,
    pc: usize,
) -> Result<(), VmError> {
    let lhs = vm.int(a, pc)?;
    vm.set(dst, Value::Int(lhs & imm));
    Ok(())
}

pub(crate) fn exec_or(vm: &mut Vm, dst: Reg, a: Reg, b: Reg, pc: usize) -> Result<(), VmError> {
    let lhs = vm.int(a, pc)?;
    let rhs = vm.int(b, pc)?;
    vm.set(dst, Value::Int(lhs | rhs));
    Ok(())
}

pub(crate) fn exec_or_imm(
    vm: &mut Vm,
    dst: Reg,
    a: Reg,
    imm: i64,
    pc: usize,
) -> Result<(), VmError> {
    let lhs = vm.int(a, pc)?;
    vm.set(dst, Value::Int(lhs | imm));
    Ok(())
}

pub(crate) fn exec_xor(vm: &mut Vm, dst: Reg, a: Reg, b: Reg, pc: usize) -> Result<(), VmError> {
    let lhs = vm.int(a, pc)?;
    let rhs = vm.int(b, pc)?;
    vm.set(dst, Value::Int(lhs ^ rhs));
    Ok(())
}

pub(crate) fn exec_xor_imm(
    vm: &mut Vm,
    dst: Reg,
    a: Reg,
    imm: i64,
    pc: usize,
) -> Result<(), VmError> {
    let lhs = vm.int(a, pc)?;
    vm.set(dst, Value::Int(lhs ^ imm));
    Ok(())
}

pub(crate) fn exec_not(vm: &mut Vm, dst: Reg, src: Reg, pc: usize) -> Result<(), VmError> {
    let val = vm.int(src, pc)?;
    vm.set(dst, Value::Int(!val));
    Ok(())
}

pub(crate) fn exec_shl(vm: &mut Vm, dst: Reg, a: Reg, b: Reg, pc: usize) -> Result<(), VmError> {
    let lhs = vm.int(a, pc)?;
    let shift = validate_shift(vm.int(b, pc)?, "shl", pc)?;
    vm.set(dst, Value::Int(lhs << shift));
    Ok(())
}

pub(crate) fn exec_shl_imm(
    vm: &mut Vm,
    dst: Reg,
    a: Reg,
    imm: i64,
    pc: usize,
) -> Result<(), VmError> {
    let lhs = vm.int(a, pc)?;
    let shift = validate_shift(imm, "shl", pc)?;
    vm.set(dst, Value::Int(lhs << shift));
    Ok(())
}

pub(crate) fn exec_shr(vm: &mut Vm, dst: Reg, a: Reg, b: Reg, pc: usize) -> Result<(), VmError> {
    let lhs = vm.int(a, pc)?;
    let shift = validate_shift(vm.int(b, pc)?, "shr", pc)?;
    vm.set(dst, Value::Int(lhs >> shift));
    Ok(())
}

pub(crate) fn exec_shr_imm(
    vm: &mut Vm,
    dst: Reg,
    a: Reg,
    imm: i64,
    pc: usize,
) -> Result<(), VmError> {
    let lhs = vm.int(a, pc)?;
    let shift = validate_shift(imm, "shr", pc)?;
    vm.set(dst, Value::Int(lhs >> shift));
    Ok(())
}
