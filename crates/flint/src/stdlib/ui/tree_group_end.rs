//! `ui.tree_group_end` — `dst = html ++ </ul></li>`. Closes a folder node
//! opened with `ui.tree_group`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.tree_group_end", "</ul></li>\n")
}
