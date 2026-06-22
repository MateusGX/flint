//! `ui.accordion_end` — `dst = html ++ </div>`. Closes an accordion opened
//! with `ui.accordion`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.accordion_end", "</div>\n")
}
