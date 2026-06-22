pub(super) const HTML_REG: &str = "r14";

pub(super) fn append_uses(out: &mut String, uses: Vec<String>) {
    let mut any = false;
    for use_path in uses {
        out.push_str("use \"");
        out.push_str(&flint_string_literal(&use_path));
        out.push_str("\"\n");
        any = true;
    }
    if any {
        out.push('\n');
    }
}

pub(super) fn append_handler_start(out: &mut String, handler: &str) {
    out.push_str(handler);
    out.push_str(":\n");
    out.push_str("    mov ");
    out.push_str(HTML_REG);
    out.push_str(", \"\"\n");
}

/// Emits the `section .route` block that declares the page's HTTP handler.
/// Called before the handler body so the generated source follows the same
/// section format as user-written `.fl` files.
pub(super) fn append_route_section(
    out: &mut String,
    method: &str,
    route_path: &str,
    handler: &str,
) {
    out.push_str("section .route\n    ");
    out.push_str(method);
    out.push_str(" \"");
    out.push_str(&flint_string_literal(route_path));
    out.push_str("\" -> ");
    out.push_str(handler);
    out.push_str("\n\n");
}

pub(super) fn emit_section_lines(src: &str, out: &mut String) {
    for line in src.lines() {
        let t = line.trim();
        if !t.is_empty() {
            out.push_str("    ");
            out.push_str(t);
            out.push('\n');
        }
    }
}

pub(super) fn append_handler_end(out: &mut String) {
    out.push_str("    ncall http.html, ");
    out.push_str(HTML_REG);
    out.push_str("\n    ret\n");
}

pub(super) fn flint_string_literal(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\t' => escaped.push_str("\\t"),
            '\r' => escaped.push_str("\\r"),
            other => escaped.push(other),
        }
    }
    escaped
}
