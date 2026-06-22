//! `ui.file` — `dst = html ++ <labeled file upload input>`. Use inside a form;
//! renders `<input type="file">`. Remember to add `enctype="multipart/form-data"`
//! on the `ui.form` action if handling uploads.

use crate::vm::NativeFn;

use super::support::labeled_input;

pub(super) fn make() -> NativeFn {
    labeled_input("ui.file", Some("file"))
}
