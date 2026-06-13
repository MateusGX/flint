//! `call`/`ret` — the function-call convention: `call` pushes a return
//! address and jumps to the target; `ret` pops it and jumps back. Always
//! discussed (and now filed) as a pair, since neither makes sense alone.
//!
//! Each function returns the instruction pointer to continue at, which the
//! dispatch loop assigns to `next_pc` — same convention as [`super::jump`].

use crate::vm::error::VmError;
use crate::vm::value::Value;
use crate::vm::Vm;

pub(crate) fn exec_call(
    vm: &mut Vm,
    target: usize,
    return_pc: usize,
    pc: usize,
) -> Result<usize, VmError> {
    vm.inc_call_depth(pc)?;
    vm.push_stack(Value::Int(return_pc as i64));
    Ok(target)
}

pub(crate) fn exec_ret(vm: &mut Vm, pc: usize) -> Result<usize, VmError> {
    vm.dec_call_depth();
    let addr = vm
        .pop_stack()
        .ok_or_else(|| VmError::new(pc, "call stack underflow"))?;
    let Value::Int(addr) = addr else {
        return Err(VmError::new(
            pc,
            "return address on the stack is not an integer (stack corrupted)",
        ));
    };
    if addr < 0 {
        return Err(VmError::new(
            pc,
            "return address is negative (stack corrupted)",
        ));
    }
    Ok(addr as usize)
}
