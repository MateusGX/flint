//! `ui.form_end` — `dst = html ++ <close form>`. Closes a form opened with
//! `ui.form`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.form_end", "</form>\n")
}
