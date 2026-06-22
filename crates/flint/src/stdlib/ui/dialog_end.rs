//! `ui.dialog_end` — `dst = html ++ <close dialog body + panel>`. Closes a
//! dialog opened with `ui.dialog`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.dialog_end", "</div></div></div>\n")
}
