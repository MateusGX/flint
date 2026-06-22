//! `ui.tree_group` — `dst = html ++ <tree folder node opener>`. An expandable
//! folder node. Add children with `ui.tree_item` or nested `ui.tree_group`,
//! then close with `ui.tree_group_end`.

use std::sync::Arc;

use crate::vm::{NativeFn, Value};

use crate::stdlib::{arg, expect_str, native};

use super::support::escape_html;

pub(super) fn make() -> NativeFn {
    native(|args| {
        let html = expect_str(arg(args, 0, "ui.tree_group")?, "ui.tree_group", 0)?;
        let label = expect_str(arg(args, 1, "ui.tree_group")?, "ui.tree_group", 1)?;
        let label = escape_html(label);
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<li class=\"flint-tree-node\"><span class=\"flint-tree-label\">{label}</span><ul class=\"flint-tree\">\n"
        )))))
    })
}
