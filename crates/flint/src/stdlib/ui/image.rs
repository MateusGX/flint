//! `ui.image` — `dst = html ++ <img>`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::{escape_attr, escape_html};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.image")?, "ui.image", 0)?;
        let src = expect_str(arg(args, 1, "ui.image")?, "ui.image", 1)?;
        let alt = expect_str(arg(args, 2, "ui.image")?, "ui.image", 2)?;
        let src = escape_attr(src);
        let alt = escape_html(alt);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<img class=\"flint-image\" src=\"{src}\" alt=\"{alt}\">\n"
        )))))
    })
}
