//! `ui.number` — `dst = html ++ <labeled number input>`. Use inside a form;
//! renders `<input type="number">`.

use crate::vm::NativeFn;

use super::support::labeled_input;

pub(super) fn make() -> NativeFn {
    labeled_input("ui.number", Some("number"))
}
