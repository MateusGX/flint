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
│       │   └── pages  — `.flint.html` and `.flint.ui` pages compiled into route modules
│       ├── stdlib/    — pure native functions (string.*, json.*, math.*, time.*, env.*, crypto.*)
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

`lang::compile_source` runs the full pipeline: `lexer::lex` → `parser::parse` → `preprocessor::expand` (inlines `use "path"` directives) → `compiler::compile`. The output is a `vm::Program` — a flat `Vec<Instr>` plus a string constant pool. `Vm::call(program, address)` executes it from a given instruction address. The HTTP layer (`http::dispatch`) creates a fresh `Vm` + `NativeRegistry` per request, registers both stdlib and per-request `http.*` natives (which close over an `Arc<Mutex<HttpExchange>>`), calls the matched handler address, and converts the resulting `HttpExchange` into an axum response.

## Pages (`pages/*.flint.html`, `pages/*.flint.ui`)

`Flint` has two server-rendered page layers. Files ending in `.flint.html` are HTML-first JSP/ASP.NET-style templates. Files ending in `.flint.ui` are control-first pages, closer to Windows Forms for the web, rendered with Flint's default CSS. Both are compiled by `lang::pages` into ordinary `.fl` route modules before the normal compiler runs. `flint serve` loads `routes/*.fl` plus `pages/**/*.flint.html` and `pages/**/*.flint.ui`; `flint build` embeds the generated route source.

Page syntax:

```html
@page "/"
@use "services/profile.fl"
<%
mov r0, "name"
ncallr r1, http.query, r0
%>
<!doctype html>
<h1>Hello <%= r1 %></h1>
```

- `@page` defaults to `GET` and can be written as `@page POST "/path"`; `@route METHOD "/path"` is also accepted.
- Without an explicit route, file paths become routes: `pages/index.flint.html` or `pages/index.flint.ui` → `/`, `pages/about.flint.html` → `/about`, `pages/users/[id].flint.html` → `/users/:id`.
- `<% ... %>` contains normal Flint instructions.
- `<%= ... %>` appends a register or literal to the response via `string.from`.
- The generated handler reserves `r14` for the HTML accumulator and `r15` as scratch while rendering.
- Pages finish with `http.html`, which sets `Content-Type: text/html; charset=utf-8` unless the handler already set one.

UI page syntax:

```txt
@page "/dashboard"
<%
mov r0, "name"
ncallr r1, http.query, r0
mov r15, "Dashboard"
ncallr r14, ui.window, r14, r15
mov r15, "A styled page without handwritten HTML."
ncallr r14, ui.text, r14, r15
mov r15, "Profile"
ncallr r14, ui.card, r14, r15
mov r15, "Name"
ncallr r14, ui.field, r14, r15, r1
mov r15, "Open API"
mov r2, "/hello"
ncallr r14, ui.button, r14, r15, r2
ncallr r14, ui.card_end, r14
ncallr r14, ui.window_end, r14
%>
```

- `.flint.ui` pages compile through the exact same pipeline as `.flint.html` —
  there is no separate control DSL. The body is just `<% ... %>` code that
  calls `ui.*` stdlib natives, each shaped `(html: str, ...) -> str`, to
  append styled HTML fragments to the `r14` accumulator.
- Available natives: `ui.window`/`ui.window_end`, `ui.card`/`ui.card_end`,
  `ui.section`/`ui.section_end`, `ui.row`/`ui.row_end`,
  `ui.column`/`ui.column_end`, `ui.title`, `ui.text`, `ui.field`, `ui.button`,
  `ui.form`/`ui.form_end`, `ui.input`, `ui.submit`. `ui.window` also emits the
  document shell, Flint's default stylesheet, and sets the browser tab title
  (`<title>`) from its `title` argument.
- `ncall`/`ncallr` arguments must be registers, so string literals passed to
  `ui.*` natives go through a scratch register first (e.g. `mov r15, "..."`
  then `ncallr r14, ui.field, r14, r15, r1`).

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

`use "services/tasks.fl"` in a `.fl` file is handled by `crates/flint/src/lang/preprocessor.rs` before compilation. It inlines the target file at that point, resolved relative to the project root (`flint.toml` directory). Files included more than once are deduplicated (first occurrence wins). Because all inlined content shares one flat label namespace, the convention is to prefix every label with the function name: `tasks_get_found:`, not `found:`.

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
- **Linear memory has 4096 slots (`MEMORY_SIZE`).** `section .text` (the default) holds code; `section .data`/`.bss` declare named, fixed-address cells (`data <value>` / `res <count>`) that are seeded into memory before the program runs. `mov reg, label` for a `.data`/`.bss` label loads its address, not its value — read/write through it with `load`/`store`.

## Project layout (flint.toml)

```toml
[project]
name    = "my-app"
version = "0.1.0"

[server]
host         = "127.0.0.1"
port         = 3000
routes       = "routes"       # the server loads *.fl from this directory
pages        = "pages"        # the server loads **/*.flint.html and **/*.flint.ui from this directory
services     = "services"     # included via `use` by route files
repositories = "repositories" # included via `use` by service files
```

`flint serve` reads `flint.toml`, then calls `lang::load_app_dir(routes_dir, project_root)` for `.fl` files and `lang::load_pages_dir(pages_dir, project_root)` for page files, registering all declared/generated routes.

## CLI binary layout

The CLI binary lives in `crates/flint-cli/src/`. The module named `build.rs`
there is a normal Rust module for `flint build`, not a Cargo build script.
