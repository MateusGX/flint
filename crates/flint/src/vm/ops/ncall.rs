//! `ncall`/`ncallr` — calling into a Rust-implemented native function by
//! name (resolved through the program's string pool and looked up in the
//! registry the `Vm` was built with), passing register values as arguments
//! and optionally writing the result back. The bridge between bytecode and
//! `crate::stdlib`/`crate::http`'s native libraries — see `native.rs`.

use crate::vm::error::VmError;
use crate::vm::instr::{Program, Reg};
use crate::vm::value::Value;
use crate::vm::Vm;

pub(crate) fn exec(
    vm: &mut Vm,
    program: &Program,
    name_idx: u32,
    args: &[Reg],
    dst: Option<Reg>,
    pc: usize,
) -> Result<(), VmError> {
    let name = vm.string_constant(program, name_idx, pc)?;
    let arg_values: Vec<Value> = args.iter().map(|r| vm.get(*r).clone()).collect();
    let result = {
        let f = vm
            .natives()
            .get(name.as_ref())
            .ok_or_else(|| VmError::new(pc, format!("unknown native function '{name}'")))?;
        f(&arg_values).map_err(|message| VmError::new(pc, message))?
    };
    if let Some(dst_reg) = dst {
        let value = result.ok_or_else(|| {
            VmError::new(
                pc,
                format!("native function '{name}' was called with 'ncallr' but returned no value"),
            )
        })?;
        vm.set(dst_reg, value);
    }
    Ok(())
}
