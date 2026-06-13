use crate::stdlib::{arg, native};
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|args| match arg(args, 0, "math.abs")? {
        Value::Int(n) => n
            .checked_abs()
            .map(Value::Int)
            .map(Some)
            .ok_or_else(|| "'math.abs' result is outside the int range".to_string()),
        Value::Float(f) => Ok(Some(Value::Float(f.abs()))),
        other => Err(format!(
            "'math.abs' expects an int or float, found '{}'",
            other.type_name()
        )),
    })
}
