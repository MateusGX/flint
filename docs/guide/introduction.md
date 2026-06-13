# Introduction

`Flint` is a small programming language and HTTP runtime. You write route
handlers in `.fl` files, HTML-first pages in `.flint.html` files, or
control-first pages in `.flint.ui` files. The CLI compiles them to bytecode and
serves them with a fresh virtual machine per request.

The language is assembly-like on purpose: every line does one visible thing.
There are registers, labels, jumps, calls, and native functions for practical
work such as HTTP, JSON, strings, math, environment variables, time, and UUIDs.

## A Complete Route

This handler responds to `GET /hello` with plain text:

```txt
say_hello:
    mov r0, "Hello!"
    ncall http.text, r0
    ret

route GET "/hello" -> say_hello
```

Read it from top to bottom:

| Line | Meaning |
|---|---|
| `say_hello:` | Defines a label the route below can target. |
| `mov r0, "Hello!"` | Stores a string in register `r0`. |
| `ncall http.text, r0` | Sets the response body to plain text. |
| `ret` | Returns from the handler. |
| `route GET "/hello" -> say_hello` | Maps an HTTP route to the handler. |

## A Complete Page

This page responds to `GET /` with HTML:

```html
@page "/"
<!doctype html>
<h1>Hello from Flint</h1>
```

Pages can include Flint instructions:

```html
@page "/hello"
<%
mov r0, "name"
ncallr r1, http.query, r0
%>
<h1>Hello <%= r1 %></h1>
```

The page compiler turns this into a normal route module that calls
`http.html`.

## What Runs

```txt
.fl routes, .flint.html pages, and .flint.ui pages
      |
      v
lexer / parser / page compiler / preprocessor
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

- There are 16 general registers: `r0` through `r15`.
- Registers are shared by all functions in a compiled module.
- Route files are compiled independently, then registered on one HTTP router.
- `use "path.fl"` inlines shared code before compilation.
- JSON is a runtime value, not a string convention.
- Server-rendered HTML and UI pages compile into ordinary Flint route handlers.

## Learning Path

1. Install and create a project: [Installation](/guide/installation).
2. Build a few endpoints: [First API](/guide/first-api).
3. Render HTML: [Visual Pages](/guide/pages) and [UI Pages](/guide/ui-pages).
4. Learn the VM model: [Core Concepts](/guide/core-concepts).
5. Split a larger app: [Project Structure](/guide/project-structure).
6. Keep the references nearby: [Language Syntax](/reference/language),
   [Instruction Set](/reference/instructions), and
   [Native Functions](/reference/native-functions).
