//! `ui.tree` — `dst = html ++ <tree list opener>`. Windows Explorer-style
//! folder tree. Add leaves with `ui.tree_item` and nested groups with
//! `ui.tree_group`/`ui.tree_group_end`. Close with `ui.tree_end`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.tree", "<ul class=\"flint-tree\">\n")
}
