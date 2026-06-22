//! `ui.accordion_item_end` — `dst = html ++ </div></div>`. Closes an
//! accordion item opened with `ui.accordion_item`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.accordion_item_end", "</div></div>\n")
}
