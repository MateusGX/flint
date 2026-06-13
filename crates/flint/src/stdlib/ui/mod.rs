//! `ui.*` — natives for building Flint UI's default styled HTML, one
//! `(html: str, ...) -> str` fragment at a time. Each native appends a piece
//! of markup (and, for `ui.window`, the document shell and default
//! stylesheet) to an accumulator string, mirroring `string.concat`. See
//! `crates/flint/src/stdlib/mod.rs` for the one-file-per-native layout.

mod button;
mod card;
mod card_end;
mod column;
mod column_end;
mod field;
mod form;
mod form_end;
mod input;
mod row;
mod row_end;
mod section;
mod section_end;
mod submit;
mod support;
mod text;
mod title;
mod window;
mod window_end;

use crate::vm::NativeRegistry;

pub fn register(registry: &mut NativeRegistry) {
    registry.register_exact("ui.window", 2, window::make());
    registry.register_exact("ui.window_end", 1, window_end::make());
    registry.register_exact("ui.card", 2, card::make());
    registry.register_exact("ui.card_end", 1, card_end::make());
    registry.register_exact("ui.section", 2, section::make());
    registry.register_exact("ui.section_end", 1, section_end::make());
    registry.register_exact("ui.row", 1, row::make());
    registry.register_exact("ui.row_end", 1, row_end::make());
    registry.register_exact("ui.column", 1, column::make());
    registry.register_exact("ui.column_end", 1, column_end::make());
    registry.register_exact("ui.title", 2, title::make());
    registry.register_exact("ui.text", 2, text::make());
    registry.register_exact("ui.field", 3, field::make());
    registry.register_exact("ui.button", 3, button::make());
    registry.register_exact("ui.form", 3, form::make());
    registry.register_exact("ui.form_end", 1, form_end::make());
    registry.register_exact("ui.input", 3, input::make());
    registry.register_exact("ui.submit", 2, submit::make());
}
