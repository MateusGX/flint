use crate::stdlib::{arg, native};
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let a = arg(args, 0, "math.min")?;
        let b = arg(args, 1, "math.min")?;
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Ok(Some(Value::Int(*x.min(y)))),
            (Value::Float(x), Value::Float(y)) => Ok(Some(Value::Float(x.min(*y)))),
            (Value::Int(x), Value::Float(y)) => Ok(Some(Value::Float((*x as f64).min(*y)))),
            (Value::Float(x), Value::Int(y)) => Ok(Some(Value::Float(x.min(*y as f64)))),
            _ => Err(format!(
                "'math.min' expects numeric arguments, found '{}' and '{}'",
                a.type_name(),
                b.type_name()
            )),
        }
    })
}
