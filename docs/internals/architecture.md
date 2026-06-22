# Architecture

This page maps the Rust code to the language and runtime behavior.

## Crate Layout

```txt
crates/flint/src/
├── vm/        bytecode format, values, native registry, interpreter
├── lang/      lexer, parser, preprocessor, compiler, page compiler
├── stdlib/    pure native functions
├── http/      HTTP router, dispatcher, exchange, http.* natives
crates/flint-cli/
└── src/       CLI commands and project templates
```

## Source Pipeline

Plain `.fl` route source goes through:

```txt
source text
  -> lang::preprocessor::validate_sections + normalize_sections + expand
  -> lang::lexer::lex
  -> lang::parser::parse
  -> lang::compiler::compile_app
  -> vm::Program + routes
```

The public convenience entry points are:

| Function | Use |
|---|---|
| `lang::compile_source` | Compile source without route metadata. |
| `lang::compile_app_source` | Compile source with route metadata. |
| `lang::load_app_dir` | Compile every direct `.fl` file in a routes directory. |

## UI Page Pipeline

Server-rendered UI pages go through an extra source-generation step:

```txt
pages/**/*.flint.ui
  -> lang::pages::compile_page_source
  -> generated .fl source
  -> preprocessor
  -> compiler
  -> vm::Program + routes
```

The page compiler does not add VM features. It emits normal Flint code that
uses `ui.*` stdlib natives and `http.html`. `.flint.ui` files are
section-based: `section .route` declares the page route, `section .text`
inserts raw Flint instructions, and `section .render` is compiled into calls
that append Flint's default styled HTML and CSS to the accumulator.

## Lexer

`crates/flint/src/lang/lexer.rs` turns source text into spanned tokens:

- identifiers
- integers
- floats
- strings
- commas, colons, brackets
- `->`
- newlines

It appends a trailing newline so the parser can treat the final line like any
other line.

## Parser

`crates/flint/src/lang/parser.rs` parses one line at a time into AST items:

- `Label`
- `Function`
- `Route`
- `Instruction`

The parser does only shape checks. It does not resolve labels, route handlers,
native names, or instruction-specific operand types beyond register and memory
syntax.

## Preprocessor

`crates/flint/src/lang/preprocessor.rs` expands:

```txt
use "path.fl"
```

Paths are resolved from the project root. Includes are recursive and
deduplicated by canonical path.

## Compiler

`crates/flint/src/lang/compiler/` is split by pass:

| File | Job |
|---|---|
| `symbols.rs` | Assign instruction addresses to labels and functions. |
| `strings.rs` | Intern string constants into the program string pool. |
| `instructions.rs` | Lower AST instructions to VM instructions. |
| `routes.rs` | Validate route methods and resolve handlers. |
| `mod.rs` | Wire passes together. |

Each route file compiles independently. There is no linker that merges modules.

## VM

`crates/flint/src/vm/` defines:

| File | Job |
|---|---|
| `instr.rs` | Bytecode instruction enum and program type. |
| `value.rs` | Runtime value enum. |
| `native.rs` | Native function type and registry. |
| `mod.rs` | VM state and dispatch loop. |
| `ops/` | Instruction behavior, grouped by action. |

The VM dispatch loop is centralized in `Vm::execute`. Instruction behavior is
delegated to plain functions under `vm::ops`.

## Native Functions

The standard library lives under `crates/flint/src/stdlib/`, one namespace per
directory.

Each native file exposes:

```rust
pub(super) fn make() -> NativeFn
```

Each namespace has a `register` function. `stdlib::register_all` registers all
pure namespaces into a `NativeRegistry`.

HTTP natives live in `crates/flint/src/http/natives/` because they close over
the current request/response exchange.

## HTTP Runtime

The HTTP flow is:

```txt
compiled modules
  -> http::router
  -> matched route
  -> http::dispatch
  -> fresh VM + natives
  -> handler call
  -> HttpResponse
  -> axum Response
```

`http::dispatch` creates one `HttpExchange` per request. The `http.*` natives
read and write that exchange through an `Arc<Mutex<_>>`.

## CLI

The CLI is under `crates/flint-cli/src/`:

| File | Job |
|---|---|
| `main.rs` | Command parsing and command dispatch. |
| `config.rs` | `flint.toml` parsing and defaults. |
| `templates.rs` | `flint new` project templates. |
| `build.rs` | Project bytecode build flow. |
| `bytecode.rs` | Portable `.flintbc` encode/decode and payload obfuscation. |
| `site.rs` | Static HTML export for UI-only projects. |
| `out.rs` | Terminal output helpers. |
| `util.rs` | String escaping helpers used by template generation. |

`flint build` compiles source modules and generated UI pages to VM-ready
bytecode, writes `dist/<project-name>.flintbc`, and stores no source text in
the artifact. `flint run <file.flintbc>` decodes that artifact into
`AppModule`s and serves it directly.

`flint build --static` loads only UI pages, executes each page handler through
the same VM/`ui.*` path with a synthetic empty `GET` request, and writes the
resulting HTML to directory-index files under `dist/`. Inline Flint UI `<style>` and `<script>` blocks are stripped from the rendered
HTML and the shared assets are written once as `dist/flint.css` and
`dist/flint.js` directly from the built-in `UI_CSS` / `UI_JS` constants.

## Extension Map

| Goal | Edit |
|---|---|
| Add an instruction | `vm/instr.rs`, `vm/ops/*`, `vm/mod.rs`, `lang/compiler/instructions.rs`. |
| Add a pure native | New file in `crates/flint/src/stdlib/<namespace>/`, register in that namespace. |
| Add an HTTP native | New file in `crates/flint/src/http/natives/`, register in `http/natives/mod.rs`. |
| Add UI page section or render syntax | `crates/flint/src/lang/pages/`. |
| Add a `ui.*` native | New file in `crates/flint/src/stdlib/ui/`, register in `stdlib/ui/mod.rs`. |
| Add route loading behavior | `crates/flint/src/lang/app.rs` or `crates/flint-cli/src/main.rs`. |
| Add CLI behavior | `crates/flint-cli/src/main.rs` and supporting module. |
