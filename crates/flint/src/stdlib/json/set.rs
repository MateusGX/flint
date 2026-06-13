//! `json.set` — `dst` = a copy of `j` with `key` (str → property, int →
//! index) set to `value`, normalizing `j` into an object/array first if
//! needed. Copy-on-write: see the module doc on [`super`] for why.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};
use serde_json::Value as Json;

use crate::stdlib::json::support::value_to_json;
use crate::stdlib::{arg, expect_json, native};

const MAX_IMPLICIT_ARRAY_INDEX: usize = 1_000_000;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let json = expect_json(arg(args, 0, "json.set")?, "json.set", 0)?;
        let key = arg(args, 1, "json.set")?;
        let value = arg(args, 2, "json.set")?;
        let mut document = json.clone();
        let inserted = value_to_json(value);
        match key {
            Value::Str(k) => {
                if !document.is_object() {
                    document = Json::Object(Default::default());
                }
                document
                    .as_object_mut()
                    .expect("just normalized to an object")
                    .insert(k.to_string(), inserted);
            }
            Value::Int(i) => {
                if !document.is_array() {
                    document = Json::Array(Vec::new());
                }
                let array = document
                    .as_array_mut()
                    .expect("just normalized to an array");
                let index = array_index(*i, "json.set")?;
                if index >= array.len() {
                    array.resize(index + 1, Json::Null);
                }
                array[index] = inserted;
            }
            _ => {
                return Err(format!(
                    "'json.set' expects a string or integer key as argument 2, found '{}'",
                    key.type_name()
                ))
            }
        }
        Ok(Some(Value::Json(Arc::new(document))))
    })
}

fn array_index(index: i64, native: &str) -> Result<usize, String> {
    if index < 0 {
        return Err(format!("'{native}' expects a non-negative array index"));
    }
    let index = usize::try_from(index)
        .map_err(|_| format!("'{native}' array index is too large: {index}"))?;
    if index > MAX_IMPLICIT_ARRAY_INDEX {
        return Err(format!(
            "'{native}' refuses to expand arrays past index {MAX_IMPLICIT_ARRAY_INDEX}"
        ));
    }
    Ok(index)
}
