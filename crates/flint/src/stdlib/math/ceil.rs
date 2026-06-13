use crate::stdlib::{arg, native};
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|args| match arg(args, 0, "math.ceil")? {
        Value::Int(n) => Ok(Some(Value::Int(*n))),
        Value::Float(f) => Ok(Some(Value::Int(checked_i64(f.ceil(), "math.ceil")?))),
        other => Err(format!(
            "'math.ceil' expects an int or float, found '{}'",
            other.type_name()
        )),
    })
}

fn checked_i64(value: f64, native: &str) -> Result<i64, String> {
    const I64_MAX_PLUS_ONE: f64 = 9_223_372_036_854_775_808.0;

    if !value.is_finite() {
        return Err(format!("'{native}' result is not finite"));
    }
    if value < i64::MIN as f64 || value >= I64_MAX_PLUS_ONE {
        return Err(format!("'{native}' result is outside the int range"));
    }
    Ok(value as i64)
}
