use std::sync::Arc;

use crate::stdlib::{arg, expect_str, native};
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let name = expect_str(arg(args, 0, "env.get")?, "env.get", 0)?;
        let value = std::env::var(name).unwrap_or_default();
        Ok(Some(Value::Str(Arc::from(value))))
    })
}
