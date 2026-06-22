# Introduction

`Flint` is a small programming language and HTTP runtime. You write HTTP
route modules in `.fl` files and server-rendered UI pages in `.flint.ui`
files. The CLI compiles both forms to bytecode and serves them with a fresh
virtual machine per request.

The language is assembly-like on purpose: every line does one visible thing.
There are registers, labels, jumps, calls, sections, and native functions for
practical work such as HTTP, JSON, strings, math, environment variables, time,
UUIDs, and UI rendering.

## A Complete Route

This handler responds to `GET /hello` with plain text:

```txt
section .route
    GET "/hello" -> say_hello

section .text
say_hello:
    mov r0, "Hello!"
    ncall http.text, r0
    ret
```

Read it from top to bottom:

| Line | Meaning |
|---|---|
| `section .route` | Starts route declarations. |
| `GET "/hello" -> say_hello` | Maps `GET /hello` to a global label. |
| `section .text` | Starts executable code. |
| `say_hello:` | Defines the route handler label. |
| `mov r0, "Hello!"` | Stores a string in register `r0`. |
| `ncall http.text, r0` | Sets the response body to plain text. |
| `ret` | Returns from the handler. |

## A Complete UI Page

UI pages live under `app/` and end with `.flint.ui`. They use sections too:

```txt
section .route
    GET "/"

section .render
    window "Home"
        text "Hello from Flint UI"
        card "Actions"
            btn "Open API", "/hello"
        end
    end
```

The page compiler turns this into a normal `.fl` route module that builds an
HTML accumulator with `ui.*` natives and returns it with `http.html`.

## What Runs

```txt
.fl route modules and .flint.ui pages
      |
      v
preprocessor / parser / page compiler
      |
      v
bytecode program plus route metadata
      |
      v
fresh VM per HTTP request
      |
      v
HTTP response
```

## What Makes Flint Different

- There are 16 registers: `r0` through `r15`.
- Registers are shared by all functions in a compiled module.
- Every `.fl` source file must use section blocks.
- Route files are compiled independently, then registered on one HTTP router.
- `use "path.fl"` inlines shared `.fl` code before compilation.
- `@use "path.fl"` at the top of a `.flint.ui` page includes shared `.fl`
  code into the generated route module.
- JSON is a runtime value, not a string convention.
- UI pages compile into ordinary Flint route handlers.

## Learning Path

1. Install and create a project: [Installation](/guide/installation).
2. Build a few endpoints: [First API](/guide/first-api).
3. Render a page: [UI Pages](/guide/ui-pages).
4. Learn the VM model: [Core Concepts](/guide/core-concepts).
5. Split a larger app: [Project Structure](/guide/project-structure).
6. Keep the references nearby: [Language Syntax](/reference/language),
   [Instruction Set](/reference/instructions), and
   [Native Functions](/reference/native-functions).
