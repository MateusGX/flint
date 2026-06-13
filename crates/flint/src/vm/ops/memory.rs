//! `load`/`store` — reading and writing the VM's linear memory by address
//! (as opposed to `mov`, which only ever touches registers). Address
//! resolution and bounds-checking (`memory_at`/`memory_store`) stay on `Vm`
//! itself, next to the `memory` field they read and grow; what's here is the
//! orchestration that makes each instruction's behavior — operand checking,
//! then the read or write — easy to find as a unit.

use crate::vm::error::VmError;
use crate::vm::instr::Reg;
use crate::vm::Vm;

pub(crate) fn exec_load(vm: &mut Vm, dst: Reg, addr_reg: Reg, pc: usize) -> Result<(), VmError> {
    let addr = vm.int(addr_reg, pc)?;
    let value = vm.memory_at(addr, pc)?;
    vm.set(dst, value);
    Ok(())
}

pub(crate) fn exec_store(
    vm: &mut Vm,
    addr_reg: Reg,
    src_reg: Reg,
    pc: usize,
) -> Result<(), VmError> {
    let addr = vm.int(addr_reg, pc)?;
    let value = vm.get(src_reg).clone();
    vm.memory_store(addr, value, pc)?;
    Ok(())
}
