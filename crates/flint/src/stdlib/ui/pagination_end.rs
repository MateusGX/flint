//! `ui.pagination_end` — `dst = html ++ </nav>`. Closes pagination opened
//! with `ui.pagination`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.pagination_end", "</nav>\n")
}
