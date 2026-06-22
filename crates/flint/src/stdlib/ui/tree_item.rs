//! `ui.tree_item` — `dst = html ++ <tree leaf>`. A clickable leaf node inside
//! `ui.tree`. For expandable folders, use `ui.tree_group`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::{escape_attr, escape_html};

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.tree_item")?, "ui.tree_item", 0)?;
        let label = expect_str(arg(args, 1, "ui.tree_item")?, "ui.tree_item", 1)?;
        let href = expect_str(arg(args, 2, "ui.tree_item")?, "ui.tree_item", 2)?;
        let label = escape_html(label);
        let href = escape_attr(href);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<li class=\"flint-tree-leaf\"><a href=\"{href}\">{label}</a></li>\n"
        )))))
    })
}
