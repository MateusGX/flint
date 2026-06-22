//! Section name constants and header-parsing helper shared across the Flint
//! language pipeline.
//!
//! Three layers of the pipeline each see a different subset of sections:
//!
//! - **Preprocessor** (`preprocessor.rs`): handles `.route` — lowers it to
//!   bare `route` directives before the compiler ever sees the source.
//! - **Compiler** (`compiler/symbols.rs`, `parser.rs`): handles `.text`,
//!   `.data`, `.bss` — the only sections that survive into the AST.
//! - **UI page compiler** (`pages/compiler.rs`): handles all of the above
//!   plus `.render`, which is compiled to `ui.*` native calls.

pub(crate) const ROUTE: &str = ".route";
pub(crate) const DATA: &str = ".data";
pub(crate) const BSS: &str = ".bss";
pub(crate) const TEXT: &str = ".text";
pub(crate) const RENDER: &str = ".render";

pub(crate) const COMPILER_SECTIONS: &[&str] = &[TEXT, DATA, BSS];
pub(crate) const PAGE_SECTIONS: &[&str] = &[ROUTE, DATA, BSS, TEXT, RENDER];

pub(crate) const HTTP_METHODS: &[&str] =
    &["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];

/// Extracts the section name from a trimmed `section .X` source line.
///
/// Returns `Some(".route")`, `Some(".text")`, etc., or `None` if the line is
/// not a `section` header. Accepts both `section .name` and the bare keyword
/// `section` (empty name → `""`).
pub(crate) fn section_name(trimmed: &str) -> Option<&str> {
    let rest = trimmed.strip_prefix("section")?;
    if rest.is_empty() || rest.starts_with(char::is_whitespace) {
        Some(rest.trim())
    } else {
        None
    }
}
