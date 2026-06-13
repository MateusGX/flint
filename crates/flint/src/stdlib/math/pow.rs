use crate::stdlib::{arg, native};
use crate::vm::{NativeFn, Value};

fn to_f64(v: &Value, name: &str, pos: usize) -> Result<f64, String> {
    match v {
        Value::Int(n) => Ok(*n as f64),
        Value::Float(f) => Ok(*f),
        other => Err(format!(
            "'{name}' expects numeric arguments, argument {} is '{}'",
            pos + 1,
            other.type_name()
        )),
    }
}

pub(super) fn make() -> NativeFn {
    native(|args| {
        let base = to_f64(arg(args, 0, "math.pow")?, "math.pow", 0)?;
        let exp = to_f64(arg(args, 1, "math.pow")?, "math.pow", 1)?;
        Ok(Some(Value::Float(base.powf(exp))))
    })
}
