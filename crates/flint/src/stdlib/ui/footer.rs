//! `ui.footer` — `dst = html ++ <page footer>`. `(html)` opens the footer
//! tag; `(html, text)` also inlines a copyright/text line and closes it
//! immediately, so `ui.footer_end` is only needed in the open form.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        if args.is_empty() || args.len() > 2 {
            return Err(format!(
                "'ui.footer' expects 1 or 2 argument(s), got {}",
                args.len()
            ));
        }
        let html = expect_str(arg(args, 0, "ui.footer")?, "ui.footer", 0)?;
        if args.len() == 2 {
            let text = expect_str(arg(args, 1, "ui.footer")?, "ui.footer", 1)?;
            let text = escape_html(text);
            Ok(Some(Value::Str(Arc::from(format!(
                "{html}<footer class=\"flint-footer\">{text}</footer>\n"
            )))))
        } else {
            Ok(Some(Value::Str(Arc::from(format!(
                "{html}<footer class=\"flint-footer\">\n"
            )))))
        }
    })
}
