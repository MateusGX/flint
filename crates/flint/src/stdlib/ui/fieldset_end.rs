//! `ui.fieldset_end` — `dst = html ++ </fieldset>`. Closes a fieldset opened
//! with `ui.fieldset`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.fieldset_end", "</fieldset>\n")
}
