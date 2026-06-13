//! `string.from` — renders any runtime value into its textual form.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, native};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let value = arg(args, 0, "string.from")?;
        Ok(Some(Value::Str(Arc::from(value.to_string()))))
    })
}
