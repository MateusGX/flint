//! `ui.ol_end` — `dst = html ++ </ol>`. Closes an ordered list opened with
//! `ui.ol`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.ol_end", "</ol>\n")
}
