use crate::stdlib::{arg, native};
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let a = arg(args, 0, "math.max")?;
        let b = arg(args, 1, "math.max")?;
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Ok(Some(Value::Int(*x.max(y)))),
            (Value::Float(x), Value::Float(y)) => Ok(Some(Value::Float(x.max(*y)))),
            (Value::Int(x), Value::Float(y)) => Ok(Some(Value::Float((*x as f64).max(*y)))),
            (Value::Float(x), Value::Int(y)) => Ok(Some(Value::Float(x.max(*y as f64)))),
            _ => Err(format!(
                "'math.max' expects numeric arguments, found '{}' and '{}'",
                a.type_name(),
                b.type_name()
            )),
        }
    })
}
