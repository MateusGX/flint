mod abs;
mod ceil;
mod floor;
mod max;
mod min;
mod pow;
mod rand_int;
mod random;
mod sqrt;

use crate::vm::NativeRegistry;

pub fn register(registry: &mut NativeRegistry) {
    registry.register_exact("math.abs", 1, abs::make());
    registry.register_exact("math.min", 2, min::make());
    registry.register_exact("math.max", 2, max::make());
    registry.register_exact("math.floor", 1, floor::make());
    registry.register_exact("math.ceil", 1, ceil::make());
    registry.register_exact("math.sqrt", 1, sqrt::make());
    registry.register_exact("math.pow", 2, pow::make());
    registry.register_exact("math.random", 0, random::make());
    registry.register_exact("math.rand_int", 2, rand_int::make());
}
