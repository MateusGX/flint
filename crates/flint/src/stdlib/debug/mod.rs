//! `debug.*` — natives for inspecting program behavior during development.
//! Not meant for production response bodies; use `http.*`/`json.*` for those.

mod print;

use crate::vm::NativeRegistry;

pub fn register(registry: &mut NativeRegistry) {
    registry.register("debug.print", print::make());
}
