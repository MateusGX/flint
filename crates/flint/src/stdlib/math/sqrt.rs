use crate::stdlib::{arg, native};
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let f = match arg(args, 0, "math.sqrt")? {
            Value::Int(n) => *n as f64,
            Value::Float(f) => *f,
            other => {
                return Err(format!(
                    "'math.sqrt' expects an int or float, found '{}'",
                    other.type_name()
                ))
            }
        };
        Ok(Some(Value::Float(f.sqrt())))
    })
}
