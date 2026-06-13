# Runtime and VM

This page describes behavior that is visible at runtime, even when it is not
part of source syntax.

## Program Model

A compiled program contains:

- a flat list of bytecode instructions
- a string constant pool
- route metadata when compiled as an app module

Labels and function names are resolved to instruction addresses during
compilation.

## VM State

Each VM has:

| State | Description |
|---|---|
| Registers | 16 values, `r0` through `r15`. |
| Stack | A value stack used by `push`, `pop`, `call`, and `ret`. |
| Memory | 4096 addressable value slots. |
| Flags | Result of the most recent `cmp`. |
| Program counter | Current instruction index. |
| Native registry | Runtime map from native name to Rust function. |

## Values

Runtime values are:

| Type | Rust representation | Notes |
|---|---|---|
| `int` | `i64` | Checked by integer arithmetic. |
| `float` | `f64` | Floating-point equality compares bit patterns internally. |
| `str` | reference-counted string | Immutable. |
| `json` | reference-counted JSON value | Manipulated through `json.*` natives. |

`string.from` uses the runtime display form:

| Type | Display |
|---|---|
| `int` | Decimal integer. |
| `float` | Rust `f64` display. |
| `str` | Raw string contents. |
| `json` | Compact JSON serialization. |

Page output expressions apply `string.escape_html` after this conversion before
appending to the HTML accumulator.

## Starting and Stopping

`Vm::run` starts at instruction `0`.

`Vm::call` starts at a specific instruction address and pushes a synthetic
return address one past the end of the program. This is how HTTP route handlers
are invoked.

Execution stops when:

- `hlt` runs
- the program counter moves past the final instruction
- the instruction budget is exceeded

`Vm::run` and HTTP handlers use a default instruction budget of 1,000,000
instructions. Use `Vm::run_with_instruction_limit` or
`Vm::call_with_instruction_limit` when embedding Flint and you need a different
budget.

## Registers

Registers are global for the lifetime of a VM run. Function calls do not create
frames or locals.

All registers start as integer `0`.

## Stack

The stack stores full runtime values.

`push` and `pop` are direct stack operations. `call` and `ret` also use the
same stack for return addresses.

The call depth limit is 1024. Exceeding it is a runtime error.

Because direct `push` and `pop` share storage with call return addresses, code
must keep stack operations balanced across calls.

## Memory

Memory has 4096 slots addressed by integer values.

```txt
mov r0, 42
mov r1, 7
store [r1], r0
load r2, [r1]
```

Behavior:

- memory grows lazily when `store` touches a new address
- unwritten addresses read as integer `0`
- negative addresses are runtime errors
- addresses `>= 4096` are runtime errors

## Flags

`cmp` writes three flags:

| Flag | Meaning |
|---|---|
| `eq` | left side equals right side |
| `lt` | left side is less than right side |
| `gt` | left side is greater than right side |

Conditional jumps read these flags. Any instruction between `cmp` and a jump
does not clear them.

## Native Registry

The VM starts with whatever natives the caller registers.

In HTTP requests, the runtime registers:

- all standard library namespaces
- request-scoped `http.*` natives

In standalone VM tests, callers can register only the natives they need.

## Error Model

Compile errors happen before a program runs. Runtime errors happen during VM
execution.

Common runtime errors:

- integer overflow in `add`, `sub`, `mul`, `div`, `mod`, or `neg`
- division by zero in `div`, `mod`, or `divf`
- wrong value type in an instruction or native call
- stack underflow
- call depth over 1024
- instruction budget exceeded
- invalid memory address
- unknown native function
- `ncallr` used with a native that returned no value

In the HTTP runtime, non-abort runtime errors become `500` responses.
