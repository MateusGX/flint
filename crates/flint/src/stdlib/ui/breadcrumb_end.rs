//! `ui.breadcrumb_end` — `dst = html ++ </nav>`. Closes a breadcrumb opened
//! with `ui.breadcrumb`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.breadcrumb_end", "</nav>\n")
}
