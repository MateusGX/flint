//! `ui.section` — `dst = html ++ <unframed content group opener with title>`.
//! Pair with `ui.section_end`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.section")?, "ui.section", 0)?;
        let title = expect_str(arg(args, 1, "ui.section")?, "ui.section", 1)?;
        let title = escape_html(title);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<section class=\"flint-section\"><h2>{title}</h2>\n"
        )))))
    })
}
