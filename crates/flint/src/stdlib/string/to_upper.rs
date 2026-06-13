use std::sync::Arc;

use crate::stdlib::{arg, expect_str, native};
use crate::vm::{NativeFn, Value};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let s = expect_str(arg(args, 0, "string.to_upper")?, "string.to_upper", 0)?;
        Ok(Some(Value::Str(Arc::from(s.to_uppercase()))))
    })
}
