//! `ui.navbar_end` — `dst = html ++ </nav>`. Closes a navbar opened with
//! `ui.navbar`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.navbar_end", "</nav>\n")
}
