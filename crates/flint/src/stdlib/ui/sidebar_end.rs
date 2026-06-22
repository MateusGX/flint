//! `ui.sidebar_end` — `dst = html ++ </div>`. Closes a sidebar opened with
//! `ui.sidebar`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.sidebar_end", "</div>\n")
}
