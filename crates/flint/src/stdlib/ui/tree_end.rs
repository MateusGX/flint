//! `ui.tree_end` — `dst = html ++ </ul>`. Closes a tree opened with
//! `ui.tree`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.tree_end", "</ul>\n")
}
