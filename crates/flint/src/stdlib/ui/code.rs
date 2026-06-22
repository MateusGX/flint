//! `ui.code` — `dst = html ++ <preformatted code block>`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.code")?, "ui.code", 0)?;
        let text = expect_str(arg(args, 1, "ui.code")?, "ui.code", 1)?;
        let text = escape_html(text);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<pre class=\"flint-code\"><code>{text}</code></pre>\n"
        )))))
    })
}
