//! `ui.menu_end` — `dst = html ++ </nav>`. Closes a menu opened with
//! `ui.menu`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.menu_end", "</nav>\n")
}
