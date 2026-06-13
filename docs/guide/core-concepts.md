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
| `r2` to `r13` | Temporary values. |
| `r14`, `r15` | Normal scratch in `.fl`; reserved by generated pages. |

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

After includes are expanded, all global labels share one namespace. Use
specific names such as `tasks_get_found` instead of `found`, or use a local
label (`.found:`) under each handler — local labels are mangled per
enclosing label and can't collide across handlers.

## Routes and Pages

A route declaration connects HTTP to a label:

```txt
list_users:
    ncallr r0, json.array
    ncall http.json, r0
    ret

route GET "/users" -> list_users
```

A visual page compiles to a generated route:

```html
@page "/"
<h1>Hello</h1>
```

Use routes when your source is mostly logic. Use pages when your source is
mostly HTML.
