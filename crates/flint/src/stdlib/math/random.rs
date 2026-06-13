use crate::stdlib::native;
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|_args| {
        use rand::Rng;
        let f: f64 = rand::thread_rng().gen();
        Ok(Some(Value::Float(f)))
    })
}
