//! `ui.toolbar_end` — `dst = html ++ </div>`. Closes a toolbar opened with
//! `ui.toolbar`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.toolbar_end", "</div>\n")
}
