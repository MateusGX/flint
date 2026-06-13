# Flint

`Flint` is an experimental project that explores what an assembly-like
language could look like when used to build HTTP APIs and server-rendered web
systems.

The language runs on a tiny register-based virtual machine. Route handlers are
written in `.fl` files, compiled to bytecode, and served through the Rust HTTP
runtime. The project also includes two page layers for web systems:

- `.flint.html` for HTML-first server-rendered templates.
- `.flint.ui` for control-first pages with a built-in default web style.

## Links

- [Documentation](https://flint.devlayer.app)
- [VS Code extension](https://marketplace.visualstudio.com/items?itemName=mateusam.flint-vscode)

## Example

```txt
say_hello:
    mov r0, "Hello from Flint!"
    ncall http.text, r0
    ret

route GET "/hello" -> say_hello
```

A styled UI page can be written without handwritten HTML, using `ui.*`
natives:

```txt
@page "/"
<%
mov r15, "Dashboard"
ncallr r14, ui.window, r14, r15
mov r15, "A server-rendered page from Flint UI natives."
ncallr r14, ui.text, r14, r15
mov r15, "Actions"
ncallr r14, ui.card, r14, r15
mov r15, "Open API"
mov r1, "/hello"
ncallr r14, ui.button, r14, r15, r1
ncallr r14, ui.card_end, r14
ncallr r14, ui.window_end, r14
%>
```

## Workspace

```txt
crates/flint      Rust library: VM, compiler, stdlib, HTTP runtime
crates/flint-cli  CLI binary: flint new, flint serve, flint build
docs              VitePress documentation
extensions/vscode VS Code language extension
```

## Commands

```sh
cargo test
cargo run --bin flint -- new my-app
cargo run --bin flint -- serve my-app
pnpm docs:build
```

## Status

This is a research and learning project. The goal is to make the runtime,
bytecode, HTTP layer, and page generation easy to inspect and evolve.
