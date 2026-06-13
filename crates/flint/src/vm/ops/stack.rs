//! `push`/`pop` — direct stack manipulation. (`call`/`ret` also touch the
//! stack, but as part of the function-call convention rather than as an end
//! in themselves — see [`super::call`].)

use crate::vm::error::VmError;
use crate::vm::instr::Reg;
use crate::vm::Vm;

pub(crate) fn exec_push(vm: &mut Vm, src: Reg) {
    let value = vm.get(src).clone();
    vm.push_stack(value);
}

pub(crate) fn exec_pop(vm: &mut Vm, dst: Reg, pc: usize) -> Result<(), VmError> {
    let value = vm
        .pop_stack()
        .ok_or_else(|| VmError::new(pc, "stack underflow"))?;
    vm.set(dst, value);
    Ok(())
}
