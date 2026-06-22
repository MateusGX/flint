//! `ui.list_end` — `dst = html ++ </ul>`. Closes a list opened with
//! `ui.list`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.list_end", "</ul>\n")
}
