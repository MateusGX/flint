//! `ui.input` — `dst = html ++ <labeled text input>`. Use inside a form
//! opened with `ui.form`.

use crate::vm::NativeFn;

use super::support::labeled_input;

pub(super) fn make() -> NativeFn {
    labeled_input("ui.input", None)
}
