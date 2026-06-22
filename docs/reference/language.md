# Language Syntax

This page describes Flint source syntax: tokens, lines, declarations, operands,
modules, includes, routes, and UI pages. For individual opcodes, see
[Instruction Set](/reference/instructions).

## Source Files

Application source code lives in `.fl` files. Code must be grouped into
section blocks. Each non-empty line is one of:

- a label
- a section directive
- a route entry inside `section .route`
- an instruction
- a `use` include directive

`use` directives, comments, and blank lines may appear before the first
section. Everything else must be inside a section.

```txt
use "services/tasks.fl"

section .route
    GET "/hello" -> say_hello

section .text
say_hello:
    mov r0, "Hello"
    ncall http.text, r0
    ret
```

Indentation is optional. Examples use four spaces after a label for
readability.

## Comments

Comments start with `;` and continue to the end of the line:

```txt
mov r0, 42 ; load the answer
```

## Identifiers

Identifiers are used for instruction mnemonics, labels, native function
names, route methods, and register names.

Allowed characters:

- first character: alphabetic, `_`, or `.` (a leading `.` denotes a label
  local to its enclosing label, see [Labels](#labels), and must be followed
  by an alphabetic character or `_`)
- later characters: alphanumeric, `_`, or `.`

Native names such as `http.json_body` and `string.to_int` are identifiers.

Instruction mnemonics are case-insensitive because the parser lowercases them:

```txt
MOV r0, 1
mov r0, 1
```

These are equivalent. Labels, native names, and route handler names are
case-sensitive.

## Registers

There are 16 registers:

```txt
r0 r1 r2 r3 r4 r5 r6 r7 r8 r9 r10 r11 r12 r13 r14 r15
```

Every register starts as integer `0`. Registers are global within one compiled
module. Function calls do not create local registers.

Common convention:

| Register | Convention |
|---|---|
| `r0` | First argument and main return value. |
| `r1` | Secondary return value, flag, or scratch. |
| `r2` to `r9` | Scratch values. |
| `r10` to `r15` | Scratch values in normal `.fl`; used by generated UI page handlers. |

The VM does not enforce this convention.

## Values and Literals

Flint has four runtime value types:

| Type | Literal or source |
|---|---|
| `int` | `42`, `-7` |
| `float` | `3.14`, `-0.5` |
| `str` | `"text"` |
| `json` | Returned by `json.*` or `http.json_body` natives. |

String literals support these escapes:

| Escape | Meaning |
|---|---|
| `\"` | Double quote. |
| `\\` | Backslash. |
| `\n` | Newline. |
| `\t` | Tab. |

There is no JSON literal syntax. Use `json.parse`, `json.object`,
`json.array`, `json.set`, and related natives.

## Operands

Instructions use operands separated by commas:

```txt
mov r0, "hello"
add r2, r0, 1
ncallr r3, string.concat, r1, r2
```

Operand kinds:

| Shape | Meaning |
|---|---|
| `r0` through `r15` | Register operand. |
| `42`, `-1` | Integer literal. |
| `3.14`, `-0.5` | Float literal. |
| `"text"` | String literal. |
| `[r3]` | Memory address held in register `r3`; valid only for `load` and `store`. |
| `name` | Identifier, usually a label or native name depending on instruction. |

`ncall` and `ncallr` arguments must be registers. Load literals into registers
first:

```txt
mov r0, "Ada"
ncall http.text, r0
```

## Labels

A label is an identifier followed by `:`. A label whose name starts with `.`
is a *local label*; any other label is a *global label*.

```txt
loop:
    add r0, r0, 1
    cmp r0, 10
    jl loop
```

`call` and `jmp` can reference a label before it's declared (forward
references). Duplicate labels are compile errors. Route handlers must target
a global label.

### Local labels

A label written `.name:` is local to the nearest preceding global label —
internally it's mangled to `global.name`, so the same local name can be
reused under different global labels without colliding. `call`/`jmp .name`
resolves within the enclosing global label's scope.

```txt
tasks_get:
    cmp r0, 0
    jl .not_found
    ret
.not_found:
    mov r0, "not found"
    ncall http.text, r0
    ret
```

Referencing `.name` with no preceding global label, or from a different
scope than where `.name:` is declared, is a compile error. Local labels are
the preferred way to avoid the cross-file naming collisions described in
[Includes](#includes) — write `.found:` under each handler instead of
prefixing every label with the handler's name.

## Sections

Application `.fl` files support four source sections: `.route`, `.text`,
`.data`, and `.bss`. They must contain at least one section block. Labels,
instructions, data declarations, bss reservations, and route entries cannot
appear before a section header.

`section .route` contains HTTP route entries and is handled by the
preprocessor. It lowers route entries to compiler route metadata before the
compiler sees the remaining `.text`, `.data`, and `.bss` sections.

`section .text`, `section .data`, and `section .bss` switch which region
subsequent labels and instructions belong to.

```txt
section .data
counter:
    data 41          ; one memory cell initialized to 41

section .bss
buffer:
    res 4            ; four memory cells, zero-initialized

section .text
main:
    mov r0, counter  ; r0 = address of `counter` (an int constant)
    load r1, [r0]    ; r1 = 41
    add r1, r1, 1
    store [r0], r1   ; counter = 42
    hlt
```

In `.text`, a label names an instruction (as described under [Labels](#labels)).
In `.data`/`.bss`, a label instead names a fixed **memory address**: each
label must be immediately followed by exactly one `data <value>` or
`res <count>` pseudo-instruction, which reserves the next memory cell(s) for
that label.

Switching sections while a `.data`/`.bss` label is still waiting for its
`data`/`res` directive is a compile error.

- `data <value>` (only valid in `.data`) reserves one memory cell initialized
  to `<value>` — an int, float, or string literal.
- `res <count>` (only valid in `.bss`) reserves `<count>` zero-initialized
  memory cells, where `<count>` is a positive integer literal.

`mov reg, label` for a `.data`/`.bss` label loads that label's **address**
(an integer) into `reg` — not the value stored there. Use `load`/`store` to
read or write through it, as in the example above. `.text` and `.data`/`.bss`
labels share one flat namespace, so a name can't be reused across sections.
Local labels (`.name:`) are only valid in `.text`.

Plain `mov reg, text_label` is not a function pointer operation. `mov` can
load registers, numbers, strings, floats, and `.data`/`.bss` addresses; use
`call`, `jmp`, and conditional jumps for code labels.

The combined size of all `.data`/`.bss` cells counts against the 4096-slot
linear memory described under [Linear Memory](instructions.md#linear-memory) —
exceeding it is a compile error.

## Routes

Routes connect HTTP methods and paths to (global) labels:

```txt
section .route
    METHOD "/path" -> handler
```

Example:

```txt
section .route
    GET "/users/:id" -> show_user

section .text
show_user:
    mov r0, "id"
    ncallr r1, http.param, r0
    ncall http.text, r1
    ret
```

Supported methods:

```txt
GET POST PUT PATCH DELETE HEAD OPTIONS
```

Methods are normalized to uppercase by the compiler. Paths must be string
literals. Dynamic path segments use `:name` and are read with `http.param`.

Route directives may appear before or after the handler function.

## Includes

`use` includes another `.fl` file before compilation:

```txt
use "services/tasks.fl"
```

Paths are resolved relative to the project root, the directory that contains
`flint.toml`.

Included files must use section blocks too; shared services and repositories
usually put labels under `section .text`. Includes are recursive. A file
included more than once is inlined only the first time. After expansion, all
global labels share one flat namespace, so prefer specific names:

```txt
tasks_get_found:
tasks_get_done:
```

instead of:

```txt
found:
done:
```

Or use [local labels](#labels) (`.found:`, `.done:`) under each handler —
they're mangled per enclosing global label and can't collide across handlers.

## Modules

`flint serve` compiles each `.fl` file directly inside the configured `api`
directory as a separate module. It does not recursively load nested `.fl`
files.

Each module has:

- one bytecode program
- one string constant pool
- zero or more route declarations

Files in `services/` or `repositories/` are not loaded by themselves. They
become part of a route module only when included with `use`.

## UI Pages

Pages live under the configured `app` directory, end in `.flint.ui`, and
compile into normal route modules before the normal Flint compiler runs.

Minimal page:

```txt
section .route
    GET "/"

section .render
    window "Dashboard"
        text "Rendered with default Flint styling."
    end
```

UI page sections:

| Section | Meaning |
|---|---|
| `section .route` | Optional page route. Lines use `METHOD "/path"` without `-> handler`. |
| `section .data` | Page string constants: `label db "value"`. |
| `section .bss` | Page memory reservations: `label res N`. |
| `section .text` | Raw Flint instructions inserted before rendering. |
| `section .render` | UI render commands such as `window`, `card`, `text`, and `btn`. |

`@use "path.fl"` may appear before the first section to include shared `.fl`
code into the generated route module. Included files must use sections.

If `section .route` is empty, the page route is inferred from the file path:

| File | Route |
|---|---|
| `app/index.flint.ui` | `GET /` |
| `app/blog/index.flint.ui` | `GET /blog` |
| `app/users/[id].flint.ui` | `GET /users/:id` |

Generated page handlers reserve `r14` for the HTML accumulator and use `r15`,
`r13`, `r12`, `r11`, and `r10` as scratch registers while rendering. See
[UI Pages](/guide/ui-pages) for examples.
