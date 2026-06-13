//! `json.get` — reads a property (`str` key) or index (`int` key) of `j`;
//! `dst = json null` if absent.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};
use serde_json::Value as Json;

use crate::stdlib::{arg, expect_json, native};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let json = expect_json(arg(args, 0, "json.get")?, "json.get", 0)?;
        let key = arg(args, 1, "json.get")?;
        let found = match key {
            Value::Str(k) => json.get(k.as_ref()),
            Value::Int(i) => {
                let index = array_index(*i, "json.get")?;
                json.get(index)
            }
            _ => {
                return Err(format!(
                    "'json.get' expects a string or integer key as argument 2, found '{}'",
                    key.type_name()
                ))
            }
        };
        let result = found.cloned().unwrap_or(Json::Null);
        Ok(Some(Value::Json(Arc::new(result))))
    })
}

fn array_index(index: i64, native: &str) -> Result<usize, String> {
    if index < 0 {
        return Err(format!("'{native}' expects a non-negative array index"));
    }
    usize::try_from(index).map_err(|_| format!("'{native}' array index is too large: {index}"))
}
