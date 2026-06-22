//! `ui.ol` — `dst = html ++ <ordered list opener>`. Numbered list. Add items
//! with `ui.ol_item`, close with `ui.ol_end`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.ol", "<ol class=\"flint-ol\">\n")
}
