//! Flint: a small assembly-like language, its register-based VM, a standard
//! library of native functions, and an HTTP server that runs Flint programs
//! as request handlers.
//!
//! Each concern lives in its own module — open [`vm`] for the bytecode
//! format and interpreter, [`lang`] for lexing/parsing/compiling source text
//! down to that bytecode, [`stdlib`] for the native functions every Flint
//! program can call, and [`http`] for the bridge that turns compiled
//! programs into HTTP route handlers.

pub mod http;
pub mod lang;
pub mod log;
pub mod stdlib;
pub mod vm;

/// Raw CSS for Flint's built-in UI theme (no `<style>` wrapper).
pub use stdlib::UI_CSS;
/// Full `<script>…</script>` block for Flint's built-in UI interactivity.
pub use stdlib::UI_JS;
