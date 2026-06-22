# Core Concepts

This page explains the mental model you need before reading larger Flint
programs.

## Registers

Flint has 16 registers:

```txt
r0 r1 r2 r3 r4 r5 r6 r7 r8 r9 r10 r11 r12 r13 r14 r15
```

Think of a register as a numbered variable. You do not declare variables.
Instructions and native calls read registers and write results back to
registers.

```txt
mov r0, "name"
mov r1, 42
mov r2, r0
```

Common convention:

| Register | Common use |
|---|---|
| `r0` | First argument and main return value. |
| `r1` | Secondary return value, flag, or scratch. |
| `r2` to `r9` | Temporary values. |
| `r10` to `r15` | Normal scratch in `.fl`; used by generated UI page handlers. |

The VM does not enforce this convention, but it keeps programs easier to read.

## Values

A register can hold:

| Type | Example or source |
|---|---|
| `int` | `42`, `-7` |
| `float` | `3.14`, `-0.5` |
| `str` | `"hello"` |
| `json` | `json.*` natives or `http.json_body` |

Instructions and natives check types at runtime. If `http.json` receives a
string instead of JSON, the request fails with a runtime error.

## One Instruction Per Line

Flint source is line-oriented:

```txt
mov r0, 1
add r0, r0, 1
ncall http.text, r0
```

There are no expressions like `a + b` inside operands. Put intermediate values
in registers.

## Sections

Application `.fl` files must use section blocks. `use`, blank lines, and
comments may appear before the first section; labels, instructions, route
entries, `data`, and `res` must be inside a section.

```txt
section .route
    GET "/hello" -> say_hello

section .text
say_hello:
    mov r0, "Hello"
    ncall http.text, r0
    ret
```

| Section | What belongs there |
|---|---|
| `section .route` | HTTP route entries: `METHOD "/path" -> handler`. |
| `section .text` | Labels and executable instructions. |
| `section .data` | Initialized memory cells: `label:` followed by `data value`. |
| `section .bss` | Zeroed memory cells: `label:` followed by `res count`. |

`section .route` is handled by the preprocessor. It becomes the compiler's
internal route metadata before `.text`, `.data`, and `.bss` are compiled.

Memory sections give names to fixed addresses:

```txt
section .data
counter:
    data 41

section .text
main:
    mov r0, counter
    load r1, [r0]
    ret
```

`mov r0, counter` loads the address of `counter`, not the stored value. Use
`load` and `store` to read or write memory through that address.

## Native Calls

Native functions are implemented by the runtime.

Use `ncall` for side effects:

```txt
ncall http.text, r0
```

Use `ncallr` when the native returns a value:

```txt
ncallr r1, json.object
```

Arguments must be registers. For the full list, see
[Native Functions](/reference/native-functions).

## Functions and Calls

The examples below are `.text` fragments; full `.fl` files still need a
`section .text` header.

Define a label with `name:`:

```txt
add_one:
    add r0, r0, 1
    ret
```

Call it with:

```txt
mov r0, 41
call add_one
; r0 is now 42
```

Functions share registers. If you need to preserve a value across a call, use
the stack:

```txt
push r0
call other_function
pop r0
```

## Labels and Jumps

A label marks a place in the program:

```txt
loop:
    add r0, r0, 1
    cmp r0, 10
    jl loop
```

`cmp` stores comparison flags. Conditional jumps such as `je`, `jne`, `jl`,
and `jge` use the most recent comparison.

## Modules and Includes

Each route file in `routes/` is compiled independently. Files in `services/`
and `repositories/` become part of a route module only when included:

```txt
use "services/tasks.fl"
```

Included `.fl` files must use section blocks too; shared functions normally
live under `section .text`.

After includes are expanded, all global labels share one namespace. Use
specific names such as `tasks_get_found` instead of `found`, or use a local
label (`.found:`) under each handler — local labels are mangled per
enclosing label and can't collide across handlers.

## Routes and UI Pages

A route declaration connects HTTP to a label:

```txt
section .route
    GET "/users" -> list_users

section .text
list_users:
    ncallr r0, json.array
    ncall http.json, r0
    ret
```

A UI page compiles to a generated route:

```txt
section .route
    GET "/"

section .render
    window "Home"
        text "Hello"
    end
```

Use routes when your source is mostly request/response logic. Use UI pages
when your source is mostly server-rendered controls.
