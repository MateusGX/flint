mod uuid;

use crate::vm::NativeRegistry;

pub fn register(registry: &mut NativeRegistry) {
    registry.register_exact("crypto.uuid", 0, uuid::make());
}
