<p align="center">
  <img src="docs/public/logo.svg" alt="Flint" width="96" />
</p>

<h1 align="center">Flint</h1>

`Flint` is an experimental project that explores what an assembly-like
language could look like when used to build HTTP APIs and server-rendered UI
pages.

The language runs on a tiny register-based virtual machine. Route handlers are
written in `.fl` files, compiled to bytecode, and served through the Rust HTTP
runtime. Server-rendered UI pages live in `.flint.ui` files and compile into
ordinary Flint route modules.

## Links

- [Documentation](https://flint.devlayer.app)
- [VS Code extension](https://marketplace.visualstudio.com/items?itemName=mateusam.flint-vscode)

## Example

```txt
section .route
    GET "/hello" -> say_hello

section .text
say_hello:
    mov r0, "Hello from Flint!"
    ncall http.text, r0
    ret
```

A styled UI page can be written without handwritten HTML:

```txt
section .route
    GET "/"

section .render
    window "Dashboard"
        text "A server-rendered page from Flint UI."
        card "Actions"
            btn "Open API", "/hello"
        end
    end
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
