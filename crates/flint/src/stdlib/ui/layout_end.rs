//! `ui.layout_end` — `dst = html ++ </div>`. Closes a layout opened with
//! `ui.layout`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.layout_end", "</div>\n")
}
