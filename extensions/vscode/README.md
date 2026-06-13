# Flint for VS Code

Language support for Flint route modules, Flint HTML pages, and Flint UI pages.

## Features

- Syntax highlighting for `.fl` source files.
- Syntax highlighting for `.flint.html` page templates.
- Syntax highlighting for `.flint.ui` pages.
- Embedded Flint highlighting inside `<% ... %>` and `<%= ... %>` blocks.
- Comment, bracket, quote, and word-boundary configuration.
- Snippets for routes, native calls, JSON responses, and pages.
- Autocomplete for instruction mnemonics, registers (`r0`-`r15`), and
  `namespace.name` native calls (`ui.*`, `http.*`, `string.*`, `json.*`,
  `math.*`, `time.*`, `env.*`, `crypto.*`, `debug.*`) with inline signatures
  and documentation.

## File Types

| File | Language mode |
|---|---|
| `*.fl` | Flint |
| `*.flint.html` | Flint HTML |
| `*.flint.ui` | Flint UI |

## Snippets

Common Flint snippets:

- `use`
- `route`
- `ncall`
- `ncallr`
- `json-response`
- `loop`
- `section`

Common Flint HTML snippets:

- `page`
- `page-method`
- `use`
- `block`
- `expr`

Common Flint UI snippets:

- `page`
- `block`
- `ui.window` / `ui.window_end`
- `ui.card` / `ui.card_end`
- `ui.field`
- `ui.form` / `ui.form_end`

## IntelliSense

Typing a namespace followed by `.` (e.g. `ui.`, `http.`, `string.`)
completes the available native functions with their call signature and a
short description. Inside a Flint code block, typing an instruction prefix
(e.g. `mo`, `ncallr`) completes the mnemonic as a snippet with operand
placeholders, and `r0`-`r15` complete as registers.

## Development

This extension has a small TypeScript backend (`src/`) that powers
autocomplete. Run `pnpm install` once, then `pnpm run compile` (or `pnpm run
watch`) to build it.

Open this folder in VS Code and press `F5` to launch an Extension Development
Host (this runs the `watch` build task automatically). Open a `.fl`,
`.flint.html`, or `.flint.ui` file in that host window to test the language
support.
