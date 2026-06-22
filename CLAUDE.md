# Flint — Claude Code guide

## What this project is

`Flint` is a register-based virtual machine with an assembly-like language (`.fl` files) designed for writing HTTP APIs. Programs declare route handlers in the language, and the runtime compiles them to bytecode and serves them via axum.

This repository is a Cargo workspace. The runtime/library crate lives at
`crates/flint` (`name = "flint"`), and the CLI binary lives at
`crates/flint-cli` (`name = "flint-cli"`, binary `flint`):

```
crates/
├── flint/
│   └── src/
│       ├── vm/        — bytecode format (Instr/Program), interpreter (Vm), value types
│       ├── lang/      — lexer → parser → compiler pipeline; preprocessor for `use`
│       │   └── pages  — `.flint.ui` pages compiled into route modules
│       ├── stdlib/    — pure native functions (debug.*, string.*, json.*, math.*, time.*, env.*, crypto.*)
│       └── http/      — axum server, per-request exchange, http.* natives
└── flint-cli/
    └── src/           — CLI: `flint new`, `flint serve`, `flint build`
```

## Commands

```bash
cargo test                    # run all tests (unit + integration + doc-tests)
cargo check                   # fast type-check without codegen
cargo build --bin flint       # build the CLI
cargo run --bin flint -- serve # serve a Flint project in the current directory
```

Tests live in `crates/flint/tests/programs.rs` (VM pipeline end-to-end) and `crates/flint/tests/server.rs` (HTTP integration). No test framework besides the standard `#[test]` attribute.

## Architecture in one paragraph

`lang::compile_app_source` runs the full pipeline for route files: `preprocessor::validate_sections` + `preprocessor::normalize_sections` + `preprocessor::expand` (inlines `use "path"` directives) → `lexer::lex` → `parser::parse` → `compiler::compile`. The lower-level `lang::compile_source` skips the preprocessor and is for plain programs without route sections. The output is a `vm::Program` — a flat `Vec<Instr>` plus a string constant pool. `Vm::call(program, address)` executes it from a given instruction address. The HTTP layer (`http::dispatch`) creates a fresh `Vm` + `NativeRegistry` per request, registers both stdlib and per-request `http.*` natives (which close over an `Arc<Mutex<HttpExchange>>`), calls the matched handler address, and converts the resulting `HttpExchange` into an axum response.

## Pages (`app/*.flint.ui`)

`Flint` has one server-rendered page layer. Files ending in `.flint.ui` are
section-based UI pages rendered with Flint's default CSS. They are compiled by
`lang::pages` into ordinary `.fl` route modules before the normal compiler
runs. `flint serve` loads `api/*.fl` plus `app/**/*.flint.ui`; `flint build`
compiles the generated route source into `.flintbc` bytecode. `flint build
--static` executes eligible `GET` UI page handlers with a synthetic request and
writes upload-ready HTML under `dist/`.

UI page syntax:

```txt
@use "services/profile.fl"

section .route
    GET "/dashboard"

section .text
    mov r0, "name"
    ncallr r1, http.query, r0

section .render
    window "Dashboard"
        text "A styled page without handwritten HTML."
        card "Profile"
            field "Name", r1
            btn "Open API", "/hello"
        end
    end
```

- `section .route` contains `METHOD "/path"` without an explicit handler.
  If empty, the compiler infers route paths from file names:
  `app/index.flint.ui` → `/`, `app/about.flint.ui` → `/about`,
  `app/users/[id].flint.ui` → `/users/:id`.
- `@use "path.fl"` is accepted only before the first section and includes
  shared `.fl` code in the generated route module.
- `section .text` contains raw Flint instructions that run before rendering.
- `section .render` contains UI commands (`window`, `card`, `field`, `btn`,
  `form`, `table`, `end`, etc.) that compile into `ui.*` native calls.
- `section .data` / `section .bss` declare named fixed-address memory cells,
  identical to their `.fl` counterparts — available in `.flint.ui` pages too.
- The generated handler reserves `r14` for the HTML accumulator and uses
  `r15`, `r13`, `r12`, `r11`, and `r10` as render scratch registers.
- Pages finish with `http.html`, which sets `Content-Type: text/html; charset=utf-8`
  unless the handler already set one.

### ui.* native reference

All natives live in `crates/flint/src/stdlib/ui/`. The stylesheet is in
`support.rs` (`UI_CSS`). The design theme is **vintage 2000s** (Windows
XP/JSP/ASP era): Verdana 11-12 px, `#c0c0c0` silver backgrounds, raised/inset
`border: 2px solid` for the 3-D effect, blue gradient headers (`#000080 →
#1084d0`), collapsed-border tables with striped rows. No border-radius, no box
shadows, no web fonts.

**Shell**

| Native | Args | Description |
|---|---|---|
| `ui.window` | `(html, title)` | Emits full doctype + `<head>` with `UI_CSS`, opens the page frame and header |
| `ui.window_end` | `(html)` | Closes frame, `</body></html>` |

**Navigation**

| Native | Args | Description |
|---|---|---|
| `ui.navbar` | `(html)` | Horizontal tab-bar opener |
| `ui.nav_item` | `(html, label, href)` | Tab link inside a navbar |
| `ui.navbar_end` | `(html)` | Closes navbar |
| `ui.menu` | `(html, title)` | Vertical sidebar navigation with blue category header |
| `ui.menu_item` | `(html, label, href)` | Menu link (normal state) |
| `ui.menu_active` | `(html, label, href)` | Menu link for the current/active page (inverted blue) |
| `ui.menu_end` | `(html)` | Closes menu |
| `ui.breadcrumb` | `(html)` | Breadcrumb trail opener |
| `ui.breadcrumb_item` | `(html, label, href)` | Link segment; CSS adds `»` separator via `::after` |
| `ui.breadcrumb_end` | `(html)` | Closes breadcrumb |
| `ui.pagination` | `(html)` | Pagination bar opener |
| `ui.page_item` | `(html, label, href)` | Page link |
| `ui.page_current` | `(html, label)` | Current page indicator (no link, inverted style) |
| `ui.pagination_end` | `(html)` | Closes pagination |

**Layout**

| Native | Args | Description |
|---|---|---|
| `ui.layout` | `(html)` | Two-column (sidebar + main) container opener |
| `ui.layout_end` | `(html)` | Closes layout |
| `ui.sidebar` | `(html)` | Left sidebar column (185 px); use inside `ui.layout` |
| `ui.sidebar_end` | `(html)` | Closes sidebar |
| `ui.main` | `(html)` | Main content column; use inside `ui.layout` after sidebar |
| `ui.main_end` | `(html)` | Closes main column |
| `ui.card` | `(html, title)` | Raised panel with blue gradient title bar |
| `ui.card_end` | `(html)` | Closes card |
| `ui.section` | `(html, title[, description])` | Unframed content group with `<h2>`; optional third arg renders a `<p>` subtitle |
| `ui.section_end` | `(html)` | Closes section |
| `ui.tabs` | `(html)` | Tabbed panel opener; add buttons with `ui.tab` |
| `ui.tab` | `(html, label, id)` | Tab button; `id` must match `ui.tab_panel` |
| `ui.tabs_body` | `(html)` | Closes tab bar, opens panels area |
| `ui.tab_panel` | `(html, id)` | Panel div for a tab |
| `ui.tab_panel_end` | `(html)` | Closes panel |
| `ui.tabs_end` | `(html)` | Closes panels area, emits inline JS for switching |
| `ui.accordion` | `(html)` | Collapsible sections container opener |
| `ui.accordion_item` | `(html, title)` | Collapsible section header + body opener (body hidden by default) |
| `ui.accordion_item_end` | `(html)` | Closes accordion item |
| `ui.accordion_end` | `(html)` | Closes accordion container; `flintAccordionToggle` JS from `ui.window` |
| `ui.row` | `(html)` | Horizontal layout (`display:table`) |
| `ui.row_end` | `(html)` | Closes row |
| `ui.column` | `(html)` | Vertical cell inside a row |
| `ui.column_end` | `(html)` | Closes column |
| `ui.toolbar` | `(html)` | Raised strip for grouping action buttons (Novo, Editar…) |
| `ui.toolbar_end` | `(html)` | Closes toolbar |
| `ui.action_bar` | `(html)` | Bottom strip for primary form actions (Salvar, Cancelar) |
| `ui.action_bar_end` | `(html)` | Closes action bar |
| `ui.divider` | `(html)` | Raised `<hr>` separator |
| `ui.footer` | `(html[, text])` | Page footer; with one arg opens a footer block (close with `ui.footer_end`); with two args emits a self-closing footer with `text` already inside |
| `ui.footer_end` | `(html)` | Closes footer (only needed when called with one arg) |

**Wizard**

| Native | Args | Description |
|---|---|---|
| `ui.steps` | `(html)` | Step indicator bar opener |
| `ui.step` | `(html, label, active)` | Step item; pass `"1"` for the current step, `"0"` otherwise |
| `ui.steps_end` | `(html)` | Closes steps bar |

**Tree**

| Native | Args | Description |
|---|---|---|
| `ui.tree` | `(html)` | Windows Explorer-style tree opener |
| `ui.tree_item` | `(html, label, href)` | Leaf node (file) |
| `ui.tree_group` | `(html, label)` | Folder node opener; nest items inside |
| `ui.tree_group_end` | `(html)` | Closes folder node |
| `ui.tree_end` | `(html)` | Closes tree |

**Content**

| Native | Args | Description |
|---|---|---|
| `ui.title` | `(html, text)` | `<h2>` heading |
| `ui.text` | `(html, text)` | `<p>` paragraph |
| `ui.field` | `(html, label, value)` | Key/value row (`<dl>`) |
| `ui.badge` | `(html, label)` | Inline blue uppercase status tag |
| `ui.alert` | `(html, kind, message)` | Message box; `kind`: `"info"` `"success"` `"warning"` `"error"` |
| `ui.status` | `(html, label, kind)` | Colored dot + label; `kind`: `"online"` `"offline"` `"busy"` `"away"` |
| `ui.progress` | `(html, value, max)` | XP-style green progress bar; args are integer strings |
| `ui.meter` | `(html, value, max)` | Level gauge; auto-colored green (≤50%), orange (≤75%), red (>75%) |
| `ui.stat` | `(html, label, value)` | KPI metric widget with large number and label |
| `ui.code` | `(html, text)` | `<pre><code>` block with monospace inset border |
| `ui.kbd` | `(html, text)` | `<kbd>` keyboard shortcut with raised button styling |
| `ui.link` | `(html, label, href)` | Inline `<a>` hyperlink (vs `ui.button` which is a raised block) |
| `ui.image` | `(html, src, alt)` | `<img>` with inset border |
| `ui.list` | `(html)` | Bullet list opener |
| `ui.list_item` | `(html, text)` | Bullet list item |
| `ui.list_end` | `(html)` | Closes bullet list |
| `ui.ol` | `(html)` | Numbered (ordered) list opener |
| `ui.ol_item` | `(html, text)` | Numbered list item |
| `ui.ol_end` | `(html)` | Closes ordered list |
| `ui.empty` | `(html, message)` | Empty-state placeholder ("Nenhum registro encontrado") |

**Table**

| Native | Args | Description |
|---|---|---|
| `ui.table` | `(html)` | Opens `<table>` with striped rows and blue header styling |
| `ui.table_end` | `(html)` | Closes table |
| `ui.caption` | `(html, text)` | Table caption; call immediately after `ui.table`, before first `ui.tr` |
| `ui.tr` | `(html)` | Opens `<tr>` |
| `ui.tr_end` | `(html)` | Closes `<tr>` |
| `ui.th` | `(html, label)` | Header cell (`<th>`) |
| `ui.td` | `(html, value)` | Data cell (`<td>`) |
| `ui.tfoot` | `(html)` | Table footer section opener; rows inside get bold silver background |
| `ui.tfoot_end` | `(html)` | Closes table footer |

**Dialogs**

Dialogs are hidden by default. Use `ui.dialog_trigger` (or any element with `onclick="flintDialog('id')"`) to show them. Close with the X button, Escape key, or clicking the overlay. The JS (`flintDialog` / `flintDialogClose`) is injected automatically by `ui.window`.

| Native | Args | Description |
|---|---|---|
| `ui.dialog` | `(html, id, title)` | Generic dialog opener; add body content with any `ui.*`, use `ui.action_bar` for footer buttons |
| `ui.dialog_end` | `(html)` | Closes generic dialog |
| `ui.dialog_trigger` | `(html, label, id)` | Raised button that opens the dialog with matching `id` |
| `ui.dialog_alert` | `(html, id, title, message)` | Self-contained alert with OK button |
| `ui.dialog_confirm` | `(html, id, title, message, action)` | Self-contained confirm; "Sim" GETs `action`, "Não" closes |
| `ui.dialog_prompt` | `(html, id, title, label, name, action)` | Self-contained prompt with text input; POSTs `name` to `action` |

**Actions**

| Native | Args | Description |
|---|---|---|
| `ui.button` | `(html, label, href)` | Raised link-button |

**Forms**

| Native | Args | Description |
|---|---|---|
| `ui.form` | `(html, method, action)` | Opens `<form>` |
| `ui.form_end` | `(html)` | Closes form |
| `ui.fieldset` | `(html, legend)` | `<fieldset>` with `<legend>` for grouping related fields |
| `ui.fieldset_end` | `(html)` | Closes fieldset |
| `ui.input` | `(html, label, name)` | Labeled text input |
| `ui.password` | `(html, label, name)` | Labeled `input type="password"` |
| `ui.number` | `(html, label, name)` | Labeled `input type="number"` |
| `ui.file` | `(html, label, name)` | Labeled `input type="file"` |
| `ui.textarea` | `(html, label, name)` | Labeled multiline textarea (4 rows) |
| `ui.select` | `(html, label, name)` | Opens labeled `<select>` |
| `ui.option` | `(html, label, value)` | `<option>` inside a select |
| `ui.select_end` | `(html)` | Closes select |
| `ui.checkbox` | `(html, label, name, value)` | Labeled checkbox |
| `ui.radio` | `(html, label, name, value)` | Labeled radio button; group by same `name` |
| `ui.hidden` | `(html, name, value)` | Hidden input field |
| `ui.submit` | `(html, label)` | Submit button |

## Extending the VM (adding an instruction)

Three files, in this order:

1. **`crates/flint/src/vm/instr.rs`** — add a variant to `Instr`. Group it with related instructions.
2. **`crates/flint/src/vm/ops/`** — add the behavior as a `pub(crate) fn exec_*` in its file (new file for a genuinely new action; existing file for a new encoding of an existing action, like `AddImm` next to `Add`).
3. **`crates/flint/src/vm/mod.rs`** — add one match arm in `Vm::execute()` that calls the new function.
4. **`crates/flint/src/lang/compiler/instructions.rs`** — add a case in `compile_instruction` to emit the new variant.

If the feature is a pure function over values with no new bytecode, add a **native** instead (see below) — no VM changes needed.

## Extending the stdlib (adding a native function)

1. Create `crates/flint/src/stdlib/<namespace>/<name>.rs` with a `pub(super) fn make() -> NativeFn` following the pattern of any existing file (e.g. `crates/flint/src/stdlib/string/trim.rs`).
2. Declare `mod <name>;` in `crates/flint/src/stdlib/<namespace>/mod.rs` and add `registry.register("namespace.name", name::make())`.
3. If it's a new namespace, also call `namespace::register(registry)` from `crates/flint/src/stdlib/mod.rs`.

Helper functions for argument validation (`arg`, `expect_str`, `expect_int`, `expect_json`, `native`) live in `crates/flint/src/stdlib/mod.rs` and are available to all stdlib submodules.

## Extending http.* natives

Same one-file-per-native layout, but in `crates/flint/src/http/natives/`. Natives here receive the `Arc<Mutex<HttpExchange>>` at construction time via their `make(exchange: &Arc<...>) -> NativeFn` signature — they close over it and read/write it when invoked. Register in `crates/flint/src/http/natives/mod.rs`.

`http.abort` terminates a handler early by returning `Err("__flint_abort__")` from the native. `dispatch.rs` detects this sentinel and skips the 500 error path, sending whatever response was assembled before the abort.

## The `use` directive (preprocessor)

`use "services/tasks.fl"` in a `.fl` file is handled by `crates/flint/src/lang/preprocessor.rs` before compilation. It inlines the target file at that point, resolved relative to the project root (`flint.toml` directory). Included `.fl` files must use section blocks too; shared code belongs under `section .text`. Files included more than once are deduplicated (first occurrence wins). Because all inlined content shares one flat label namespace, the convention is to prefix every label with the function name: `tasks_get_found:`, not `found:`.

## Value types

The VM has four value types in `crates/flint/src/vm/value.rs`:

| Type | Rust variant | Notes |
|---|---|---|
| `int` | `Value::Int(i64)` | 64-bit signed integer |
| `float` | `Value::Float(f64)` | IEEE 754 double; `PartialEq` uses bit comparison |
| `str` | `Value::Str(Arc<str>)` | Immutable, reference-counted |
| `json` | `Value::Json(Arc<serde_json::Value>)` | Copy-on-write via `Arc` |

`Value` is `Send + Sync` because the HTTP layer shares `Program` across worker threads.

## Key invariants

- **One VM per request.** `Vm::new` is cheap (lazy memory allocation). Never share a `Vm` across requests.
- **Registers are global within a program.** There are 16 (`r0`–`r15`). The convention is: first argument in `r0`, return value in `r0`, secondary returns in `r1`/`r2`. Push/pop to preserve values across calls.
- **Call stack depth is limited to 1024.** Deeper recursion returns a runtime error. Tracked via `Vm::inc_call_depth` / `Vm::dec_call_depth`.
- **Label namespace is flat per compiled file.** After `use` expansion, every global label (`name:`) in the file and all its transitive includes must be unique. Duplicate labels are a compile error. A label starting with `.` (e.g. `.found:`) is local to the nearest preceding global label and is mangled to `global.found` — the preferred alternative to the `tasks_get_found`-style naming convention for avoiding cross-file collisions. Only global labels can be `route` handlers.
- **`json.set` / `json.push` are copy-on-write.** They return a new document; the original is unchanged.
- **Linear memory has 4096 slots (`MEMORY_SIZE`).** `section .text` holds code; `section .data`/`.bss` declare named, fixed-address cells (`data <value>` / `res <count>`) that are seeded into memory before the program runs. `.fl` files must use section blocks. `mov reg, label` for a `.data`/`.bss` label loads its address, not its value — read/write through it with `load`/`store`.

## Project layout (flint.toml)

```toml
[project]
name    = "my-app"
version = "0.1.0"

[server]
host         = "127.0.0.1"
port         = 3000
routes       = "api"          # the server loads *.fl from this directory
pages        = "app"          # the server loads **/*.flint.ui from this directory
services     = "services"     # included via `use` by route files
repositories = "repositories" # included via `use` by service files
components   = "components"   # included via `@use` by .flint.ui pages
log          = "info"         # off | error | warn | info | debug
```

`flint serve` reads `flint.toml`, then calls `lang::load_app_dir(routes_dir, project_root)` for `.fl` files when the route directory exists and `lang::load_pages_dir(pages_dir, project_root)` for page files, registering all declared/generated routes. `flint serve file.flintbc` / `flint run file.flintbc` skips source loading and serves the compiled bytecode artifact.

## CLI binary layout

The CLI binary lives in `crates/flint-cli/src/`. Key modules:

- `build.rs` — implements `flint build` (bytecode compilation); normal Rust module, not a Cargo build script
- `site.rs` — implements `flint build --static` (static HTML export); also a normal Rust module
- `bytecode.rs` — reads `.flintbc` files for `flint serve <file.flintbc>` / `flint run <file.flintbc>`
- `config.rs` — parses `flint.toml`
- `templates.rs` — scaffold file trees for `flint new --template <name>` (`minimal`, `tasks`, `static`)
- `out.rs` — terminal output helpers (step/error/done formatting, ANSI codes)
