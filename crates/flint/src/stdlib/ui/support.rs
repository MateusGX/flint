//! Shared HTML escaping and the default stylesheet used by `ui.*` natives.

pub(super) fn escape_html(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            other => escaped.push(other),
        }
    }
    escaped
}

pub(super) fn escape_attr(text: &str) -> String {
    escape_html(text)
}

pub(super) const UI_CSS: &str = r#":root {
    color-scheme: light;
    --flint-bg: #f6f7f9;
    --flint-surface: #ffffff;
    --flint-border: #d8dee8;
    --flint-text: #1f2937;
    --flint-muted: #64748b;
    --flint-accent: #0f766e;
    --flint-accent-strong: #115e59;
    --flint-focus: #f59e0b;
}
* { box-sizing: border-box; }
body {
    margin: 0;
    min-height: 100vh;
    font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    color: var(--flint-text);
    background: var(--flint-bg);
}
.flint-window {
    width: min(980px, calc(100% - 32px));
    margin: 48px auto;
}
.flint-surface {
    background: var(--flint-surface);
    border: 1px solid var(--flint-border);
    border-radius: 8px;
    box-shadow: 0 18px 50px rgba(15, 23, 42, 0.08);
    overflow: hidden;
}
.flint-header {
    padding: 28px 32px 24px;
    border-bottom: 1px solid var(--flint-border);
    background: linear-gradient(180deg, #ffffff 0%, #f8fafc 100%);
}
.flint-eyebrow {
    margin: 0 0 8px;
    color: var(--flint-accent);
    font-size: 12px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
}
h1, h2, h3, p { margin-top: 0; }
h1 { margin-bottom: 0; font-size: 32px; line-height: 1.15; }
h2 { margin-bottom: 12px; font-size: 20px; line-height: 1.25; }
.flint-stack {
    display: grid;
    gap: 18px;
    padding: 24px 32px 32px;
}
.flint-card {
    border: 1px solid var(--flint-border);
    border-radius: 8px;
    padding: 18px;
    background: #ffffff;
}
.flint-section {
    display: grid;
    gap: 12px;
}
.flint-row {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
    align-items: center;
}
.flint-column {
    display: grid;
    gap: 12px;
}
.flint-text {
    margin-bottom: 0;
    color: var(--flint-muted);
    line-height: 1.6;
}
.flint-field {
    display: grid;
    grid-template-columns: minmax(120px, 220px) 1fr;
    gap: 12px;
    align-items: baseline;
    margin: 0;
    padding: 10px 0;
    border-top: 1px solid #eef2f7;
}
.flint-field:first-child { border-top: 0; }
.flint-field dt {
    color: var(--flint-muted);
    font-size: 13px;
    font-weight: 700;
}
.flint-field dd {
    margin: 0;
    font-weight: 600;
}
.flint-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-height: 38px;
    padding: 0 14px;
    border: 1px solid var(--flint-accent);
    border-radius: 6px;
    color: #ffffff;
    background: var(--flint-accent);
    font-weight: 700;
    text-decoration: none;
}
.flint-button:hover { background: var(--flint-accent-strong); }
.flint-form {
    display: grid;
    gap: 14px;
}
.flint-input {
    display: grid;
    gap: 6px;
    color: var(--flint-muted);
    font-size: 13px;
    font-weight: 700;
}
.flint-input input {
    width: 100%;
    min-height: 38px;
    border: 1px solid var(--flint-border);
    border-radius: 6px;
    padding: 0 10px;
    color: var(--flint-text);
    font: inherit;
}
.flint-button:focus-visible,
.flint-input input:focus {
    outline: 3px solid color-mix(in srgb, var(--flint-focus) 35%, transparent);
    outline-offset: 2px;
}
@media (max-width: 640px) {
    .flint-window { width: min(100% - 20px, 980px); margin: 20px auto; }
    .flint-header, .flint-stack { padding-left: 18px; padding-right: 18px; }
    .flint-field { grid-template-columns: 1fr; gap: 4px; }
}
"#;
