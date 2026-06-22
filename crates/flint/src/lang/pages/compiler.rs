//! Section-based compiler for `.flint.ui` pages.
//!
//! Source format:
//! ```text
//! @use "path"           (optional, top of file)
//!
//! section .route
//!     GET "/path"
//!
//! section .data
//!     label  db "string value"
//!
//! section .bss
//!     label  res N
//!
//! section .text
//!     ; raw Flint instructions (logic, HTTP queries, etc.)
//!
//! section .render
//!     window "Title"
//!       card "Card"
//!         field "Nome", r1
//!       end
//!     end
//! ```

use std::collections::HashMap;
use std::path::Path;

use super::paths::{handler_name, infer_route_path};
use super::source::{
    append_handler_end, append_handler_start, append_route_section, append_uses,
    emit_section_lines,
};
use super::{CompiledPageSource, PageCompileError};

// r14 = HTML accumulator; scratch registers for string-literal args.
// Private constants — accessible to child modules via `super::`.
pub(super) const HTML: &str = "r14";
pub(super) const S1: &str = "r15";
pub(super) const S2: &str = "r13";
pub(super) const S3: &str = "r12";
pub(super) const S4: &str = "r11";
pub(super) const S5: &str = "r10";

pub(super) const FLINT_MNEMONICS: &[&str] = &[
    "mov", "add", "sub", "mul", "div", "mod", "and", "or", "not", "xor", "shl", "shr", "neg",
    "jmp", "je", "jne", "jlt", "jgt", "jle", "jge", "call", "ret", "hlt", "push", "pop", "load",
    "store", "inc", "dec", "ncall", "ncallr", "cmp",
];

#[derive(Default)]
pub(super) struct Parsed {
    pub(super) uses: Vec<String>,
    pub(super) method: Option<String>,
    pub(super) path: Option<String>,
    pub(super) data: HashMap<String, String>,
    pub(super) bss: Vec<(String, usize)>,
    pub(super) text: String,
    pub(super) render: String,
}

pub(super) fn compile(
    source: &str,
    page_path: &Path,
    pages_dir: &Path,
) -> Result<CompiledPageSource, PageCompileError> {
    let parsed = super::parser::parse_sections(source)?;

    let route_path = parsed
        .path
        .unwrap_or_else(|| infer_route_path(page_path, pages_dir));
    let method = parsed.method.unwrap_or_else(|| "GET".to_string());
    let handler = handler_name(page_path, pages_dir);

    let mut out = String::new();

    // use directives must appear before any section block so normalize_sections
    // does not see them while in_route = true and incorrectly prefix them.
    append_uses(&mut out, parsed.uses);

    append_route_section(&mut out, &method, &route_path, &handler);

    if !parsed.bss.is_empty() {
        out.push_str("section .bss\n");
        for (label, count) in &parsed.bss {
            out.push_str(&format!("    {label} res {count}\n"));
        }
        out.push('\n');
    }

    out.push_str("section .text\n");
    append_handler_start(&mut out, &handler);

    emit_section_lines(&parsed.text, &mut out);

    super::render::compile_render(&parsed.render, &parsed.data, &mut out)?;

    append_handler_end(&mut out);

    Ok(CompiledPageSource {
        route_path,
        method,
        source: out,
    })
}
