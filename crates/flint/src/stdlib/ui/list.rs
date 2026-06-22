//! `ui.list` — `dst = html ++ <ul opener>`. Styled bullet list. Add items
//! with `ui.list_item`, close with `ui.list_end`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.list", "<ul class=\"flint-list\">\n")
}
