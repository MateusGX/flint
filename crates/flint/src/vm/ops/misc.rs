use std::sync::Arc;

use crate::vm::{error::VmError, instr::Reg, value::Value, Vm};

pub(crate) fn exec_neg(vm: &mut Vm, dst: Reg, src: Reg, pc: usize) -> Result<(), VmError> {
    let result = match vm.get(src) {
        Value::Int(n) => {
            let n = *n;
            Value::Int(
                n.checked_neg()
                    .ok_or_else(|| VmError::new(pc, "integer overflow in 'neg'"))?,
            )
        }
        Value::Float(f) => Value::Float(-f),
        other => {
            return Err(VmError::new(
                pc,
                format!(
                    "expected a numeric value in r{src} for 'neg', found '{}'",
                    other.type_name()
                ),
            ))
        }
    };
    vm.set(dst, result);
    Ok(())
}

pub(crate) fn exec_typeof(vm: &mut Vm, dst: Reg, src: Reg) {
    let name: Arc<str> = vm.get(src).type_name().into();
    vm.set(dst, Value::Str(name));
}
