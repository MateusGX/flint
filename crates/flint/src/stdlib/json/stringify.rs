//! `json.stringify` — serializes `j` (`json`) back into a `str`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_json, native};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let json = expect_json(arg(args, 0, "json.stringify")?, "json.stringify", 0)?;
        Ok(Some(Value::Str(Arc::from(json.to_string()))))
    })
}
