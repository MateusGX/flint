# Flint for VS Code

Language support for Flint route modules and Flint UI pages.

## Features

- Syntax highlighting for `.fl` source files.
- Syntax highlighting for section-based `.flint.ui` pages.
- Comment, bracket, quote, and word-boundary configuration.
- Snippets for `section .route`, route handlers, native calls, JSON responses,
  and UI render sections.
- Autocomplete for instruction mnemonics, registers (`r0`-`r15`), and
  `namespace.name` native calls (`ui.*`, `http.*`, `string.*`, `json.*`,
  `math.*`, `time.*`, `env.*`, `crypto.*`, `debug.*`) with inline signatures
  and documentation.
- Autocomplete for `use`/`@use` paths, section names, HTTP methods, local
  labels, and Flint UI render commands.
- File icons for `.fl`, `.flint.ui`, `.flintbc`, and `flint.toml` through the
  bundled `Flint Icons` file icon theme.

## File Types

| File | Language mode |
|---|---|
| `*.fl` | Flint |
| `*.flint.ui` | Flint UI |
| `*.flintbc` | Flint bytecode artifact |
| `flint.toml` | Flint project manifest |

## Snippets

Common Flint snippets:

- `use`
- `route`
- `route-section`
- `text-section`
- `ncall`
- `ncallr`
- `json-response`
- `loop`
- `section`

Common Flint UI snippets:

- `page`
- `route`
- `render`
- `text-section`
- `window`
- `block`
- `card`
- `field`
- `form`
- `btn`

## IntelliSense

Typing a namespace followed by `.` (e.g. `ui.`, `http.`, `string.`)
completes the available native functions with their call signature and a
short description. Inside a Flint code block, typing an instruction prefix
(e.g. `mo`, `ncallr`) completes the mnemonic as a snippet with operand
placeholders, and `r0`-`r15` complete as registers.

`use "` and `@use "` complete project-relative `.fl` files and folders.
`section` completes the current section set, `.route` completes HTTP methods,
and route targets / jump targets complete labels declared in the current file.
In `.flint.ui`, `section .render` completes the current render DSL commands
such as `window`, `card`, `field`, `btn`, `form`, and `end`.

## Development

This extension has a small TypeScript backend (`src/`) that powers
autocomplete. Run `pnpm install` once, then `pnpm run compile` (or `pnpm run
watch`) to build it.

Open this folder in VS Code and press `F5` to launch an Extension Development
Host (this runs the `watch` build task automatically). Open a `.fl` or
`.flint.ui` file in that host window to test the language support.
