//! `json.to_str` — `dst` = `j` as a `str`; runtime error if `j` isn't a JSON
//! string.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_json, native};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let json = expect_json(arg(args, 0, "json.to_str")?, "json.to_str", 0)?;
        let s = json
            .as_str()
            .ok_or_else(|| "json.to_str: value is not a string".to_string())?;
        Ok(Some(Value::Str(Arc::from(s))))
    })
}
