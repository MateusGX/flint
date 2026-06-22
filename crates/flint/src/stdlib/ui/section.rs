//! `ui.section` — `dst = html ++ <unframed content group opener with title>`.
//! `(html, title)` or `(html, title, description)` — the optional third arg
//! renders a `<p>` subtitle below the `<h2>`. Pair with `ui.section_end`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        if args.len() < 2 || args.len() > 3 {
            return Err(format!(
                "'ui.section' expects 2 or 3 argument(s), got {}",
                args.len()
            ));
        }
        let html = expect_str(arg(args, 0, "ui.section")?, "ui.section", 0)?;
        let title = expect_str(arg(args, 1, "ui.section")?, "ui.section", 1)?;
        let title = escape_html(title);
        let desc = if args.len() == 3 {
            let d = expect_str(arg(args, 2, "ui.section")?, "ui.section", 2)?;
            if d.is_empty() {
                String::new()
            } else {
                format!("<p class=\"flint-text\">{}</p>\n", escape_html(d))
            }
        } else {
            String::new()
        };
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<section class=\"flint-section\"><h2>{title}</h2>\n{desc}"
        )))))
    })
}
