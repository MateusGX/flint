//! `ui.footer_end` — `dst = html ++ </footer>`. Closes a footer opened with
//! `ui.footer`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.footer_end", "</footer>\n")
}
