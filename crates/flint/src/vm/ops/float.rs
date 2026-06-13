use crate::vm::{error::VmError, instr::Reg, value::Value, Vm};

fn to_float(v: &Value, reg: Reg, pc: usize) -> Result<f64, VmError> {
    match v {
        Value::Int(n) => Ok(*n as f64),
        Value::Float(f) => Ok(*f),
        _ => Err(VmError::new(
            pc,
            format!(
                "expected a numeric value in r{reg}, found '{}'",
                v.type_name()
            ),
        )),
    }
}

pub(crate) fn exec_load_float(vm: &mut Vm, dst: Reg, value: f64) {
    vm.set(dst, Value::Float(value));
}

pub(crate) fn exec_addf(vm: &mut Vm, dst: Reg, a: Reg, b: Reg, pc: usize) -> Result<(), VmError> {
    let lhs = to_float(vm.get(a), a, pc)?;
    let rhs = to_float(vm.get(b), b, pc)?;
    vm.set(dst, Value::Float(lhs + rhs));
    Ok(())
}

pub(crate) fn exec_subf(vm: &mut Vm, dst: Reg, a: Reg, b: Reg, pc: usize) -> Result<(), VmError> {
    let lhs = to_float(vm.get(a), a, pc)?;
    let rhs = to_float(vm.get(b), b, pc)?;
    vm.set(dst, Value::Float(lhs - rhs));
    Ok(())
}

pub(crate) fn exec_mulf(vm: &mut Vm, dst: Reg, a: Reg, b: Reg, pc: usize) -> Result<(), VmError> {
    let lhs = to_float(vm.get(a), a, pc)?;
    let rhs = to_float(vm.get(b), b, pc)?;
    vm.set(dst, Value::Float(lhs * rhs));
    Ok(())
}

pub(crate) fn exec_divf(vm: &mut Vm, dst: Reg, a: Reg, b: Reg, pc: usize) -> Result<(), VmError> {
    let lhs = to_float(vm.get(a), a, pc)?;
    let rhs = to_float(vm.get(b), b, pc)?;
    if rhs == 0.0 {
        return Err(VmError::new(pc, "division by zero in 'divf'"));
    }
    vm.set(dst, Value::Float(lhs / rhs));
    Ok(())
}
