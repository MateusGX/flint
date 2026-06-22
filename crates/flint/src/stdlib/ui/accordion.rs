//! `ui.accordion` — `dst = html ++ <accordion container opener>`. Add
//! collapsible sections with `ui.accordion_item`/`ui.accordion_item_end`,
//! close with `ui.accordion_end`. Clicking a header toggles it via inline JS
//! injected by `ui.window`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.accordion", "<div class=\"flint-accordion\">\n")
}
