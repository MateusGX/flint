//! `ui.pagination` — `dst = html ++ <pagination nav opener>`. Add page links
//! with `ui.page_item` and `ui.page_current`, close with `ui.pagination_end`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.pagination", "<nav class=\"flint-pagination\">")
}
