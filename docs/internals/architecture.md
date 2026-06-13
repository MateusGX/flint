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
  -> lang::lexer::lex
  -> lang::parser::parse
  -> lang::preprocessor::expand
  -> lang::compiler::compile_app
  -> vm::Program + routes
```

The public convenience entry points are:

| Function | Use |
|---|---|
| `lang::compile_source` | Compile source without route metadata. |
| `lang::compile_app_source` | Compile source with route metadata. |
| `lang::load_app_dir` | Compile every direct `.fl` file in a routes directory. |

## Page Pipeline

Server-rendered pages go through an extra source-generation step:

```txt
pages/**/*.flint.html
pages/**/*.flint.ui
  -> lang::pages::compile_page_source
  -> generated .fl source
  -> preprocessor
  -> compiler
  -> vm::Program + routes
```

The page compiler does not add VM features. It emits normal Flint code that
uses `string.concat`, `string.from`, and `http.html`. `.flint.html` and
`.flint.ui` pages compile through the exact same path — there is no separate
UI mode. By convention, `.flint.ui` pages write their body entirely as `<% %>`
code that calls `ui.*` stdlib natives (see [Native Functions](/reference/native-functions))
to append Flint's default styled HTML and CSS to the accumulator.

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
| `build.rs` | Standalone release build generation. |
| `out.rs` | Terminal output helpers. |

`flint build` generates a Rust project under `.flint-build/` and embeds all
route source and generated page source as raw strings.

## Extension Map

| Goal | Edit |
|---|---|
| Add an instruction | `vm/instr.rs`, `vm/ops/*`, `vm/mod.rs`, `lang/compiler/instructions.rs`. |
| Add a pure native | New file in `crates/flint/src/stdlib/<namespace>/`, register in that namespace. |
| Add an HTTP native | New file in `crates/flint/src/http/natives/`, register in `http/natives/mod.rs`. |
| Add page directive syntax | `crates/flint/src/lang/pages/`. |
| Add a `ui.*` native | New file in `crates/flint/src/stdlib/ui/`, register in `stdlib/ui/mod.rs`. |
| Add route loading behavior | `crates/flint/src/lang/app.rs` or `crates/flint-cli/src/main.rs`. |
| Add CLI behavior | `crates/flint-cli/src/main.rs` and supporting module. |
