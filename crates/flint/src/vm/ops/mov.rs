//! `mov` — copying a value into a register. The compiler picks one of three
//! encodings based on the source operand's shape — `LoadInt` for integer
//! literals, `LoadStr` for string literals (resolved through the program's
//! string pool), `Mov` for another register — but they're a single action
//! from the Flint programmer's point of view: `mov dst, src`.

use crate::vm::error::VmError;
use crate::vm::instr::{Program, Reg};
use crate::vm::value::Value;
use crate::vm::Vm;

pub(crate) fn exec_load_int(vm: &mut Vm, dst: Reg, value: i64) {
    vm.set(dst, Value::Int(value));
}

pub(crate) fn exec_load_str(
    vm: &mut Vm,
    program: &Program,
    dst: Reg,
    idx: u32,
    pc: usize,
) -> Result<(), VmError> {
    let s = vm.string_constant(program, idx, pc)?;
    vm.set(dst, Value::Str(s));
    Ok(())
}

pub(crate) fn exec_mov(vm: &mut Vm, dst: Reg, src: Reg) {
    let value = vm.get(src).clone();
    vm.set(dst, value);
}
