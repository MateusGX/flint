//! `ui.table` — `dst = html ++ <table opener>`. Pair with `ui.table_end`.
//! Rows are added with `ui.tr`/`ui.tr_end`; cells with `ui.th` and `ui.td`.

use crate::vm::NativeFn;

use super::support::append_literal;

pub(super) fn make() -> NativeFn {
    append_literal("ui.table", "<table class=\"flint-table\">\n")
}
