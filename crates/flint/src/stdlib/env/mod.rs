mod get;

use crate::vm::NativeRegistry;

pub fn register(registry: &mut NativeRegistry) {
    registry.register_exact("env.get", 1, get::make());
}
