# Instruction Set

This page lists every source instruction accepted by the compiler.

Operands are written as:

| Name | Meaning |
|---|---|
| `dst` | Destination register. |
| `src`, `a`, `b` | Source registers. |
| `imm` | Integer literal. |
| `f` | Float literal. |
| `target` | Label or function name. |
| `native` | Native function name such as `http.text`. |

## Data Movement

### `mov`

```txt
mov dst, src
mov dst, int
mov dst, float
mov dst, "str"
```

Copies a value into a register.

```txt
mov r0, 42
mov r1, 3.14
mov r2, "hello"
mov r3, r2
```

The source may be a register, integer literal, float literal, or string
literal.

## Integer Arithmetic

Integer arithmetic uses this shape:

```txt
op dst, a, b
op dst, a, imm
```

`a` must be a register. The right operand may be a register or integer
literal.

| Instruction | Effect |
|---|---|
| `add dst, a, b` | `dst = a + b` |
| `sub dst, a, b` | `dst = a - b` |
| `mul dst, a, b` | `dst = a * b` |
| `div dst, a, b` | Integer division. |
| `mod dst, a, b` | Integer remainder. |

Examples:

```txt
add r0, r0, 1
mul r2, r0, r1
div r3, r2, 10
```

`add`, `sub`, `mul`, `div`, and `mod` use checked integer operations and fail
on overflow. `div` and `mod` also fail on division by zero.

## Floating-Point Arithmetic

Floating-point arithmetic uses registers for every operand:

```txt
op dst, a, b
```

| Instruction | Effect |
|---|---|
| `addf dst, a, b` | `dst = a + b` |
| `subf dst, a, b` | `dst = a - b` |
| `mulf dst, a, b` | `dst = a * b` |
| `divf dst, a, b` | `dst = a / b` |

Operands may be `int` or `float`; integers are promoted to floats. Other types
are runtime errors. `divf` fails when the right operand is `0.0`.

```txt
mov r0, 3.14
mov r1, 2
addf r2, r0, r1
```

## Bitwise Operations

Bitwise operations work on integers.

| Instruction | Effect |
|---|---|
| `and dst, a, b` | Bitwise AND. |
| `or dst, a, b` | Bitwise OR. |
| `xor dst, a, b` | Bitwise XOR. |
| `not dst, src` | Bitwise NOT. |
| `shl dst, a, b` | Shift left. |
| `shr dst, a, b` | Arithmetic shift right. |

Like integer arithmetic, `and`, `or`, `xor`, `shl`, and `shr` accept a register
or integer literal as the right operand:

```txt
and r1, r0, 255
shl r2, r1, 4
not r3, r2
```

Shift amounts must be in the range `0..63`.

## Numeric and Type Helpers

| Instruction | Effect |
|---|---|
| `neg dst, src` | Negates an `int` or `float`. |
| `typeof dst, src` | Stores `"int"`, `"float"`, `"str"`, or `"json"` in `dst`. |

```txt
neg r1, r0
typeof r2, r1
```

`neg` fails on integer overflow and on non-numeric values.

## Comparison and Jumps

`cmp` compares integers and updates VM flags:

```txt
cmp a, b
cmp a, imm
```

`a` must be a register. The right operand may be a register or integer
literal.

Conditional jumps read the flags set by the most recent `cmp`.

| Instruction | Jumps when |
|---|---|
| `jmp target` | Always. |
| `je target` | Equal. |
| `jne target` | Not equal. |
| `jl target` | Less than. |
| `jg target` | Greater than. |
| `jle target` | Less than or equal. |
| `jge target` | Greater than or equal. |

Example:

```txt
mov r0, 0
loop:
    add r0, r0, 1
    cmp r0, 10
    jl loop
```

If a conditional jump runs before any `cmp`, it reads the default flags, where
all conditions are false except `jne` is true because `eq` is false.

## Stack

| Instruction | Effect |
|---|---|
| `push src` | Pushes a register value onto the VM stack. |
| `pop dst` | Pops the top stack value into a register. |

```txt
push r0
call may_change_r0
pop r0
```

`pop` fails on stack underflow.

## Calls and Returns

| Instruction | Effect |
|---|---|
| `call target` | Pushes a return address and jumps to a label. |
| `ret` | Pops the return address and jumps back. |

```txt
mov r0, 41
call add_one
hlt

add_one:
    add r0, r0, 1
    ret
```

The call depth limit is 1024. Going deeper is a runtime error.

`ret` is also how HTTP handlers return to the server. The HTTP dispatcher
starts handlers as if they had been called from outside the program.

## Linear Memory

| Instruction | Effect |
|---|---|
| `load dst, [addr]` | Loads `memory[register addr]` into `dst`. |
| `store [addr], src` | Stores `src` into `memory[register addr]`. |

Example:

```txt
mov r0, 42
mov r1, 7
store [r1], r0
load r2, [r1]
```

Memory has 4096 addressable slots. Addresses are integers. Negative addresses
and addresses greater than or equal to 4096 are runtime errors. Reading an
unwritten address returns integer `0`.

`data <value>` and `res <count>` are pseudo-instructions — they emit no
opcode, and are only valid in `section .data`/`section .bss` respectively,
immediately following a label. They reserve and (for `data`) initialize
memory cells at compile time, addressable via `load`/`store` once `mov`ed
into a register. See [Sections](language.md#sections).

## Native Calls

Native calls bridge Flint bytecode to Rust functions registered at runtime.

```txt
ncall native, arg1, arg2
ncallr dst, native, arg1, arg2
```

Use `ncall` when you only need side effects:

```txt
ncall http.text, r0
```

Use `ncallr` when the native returns a value:

```txt
ncallr r1, json.object
```

Rules:

- The native name is an identifier.
- Arguments must be registers.
- `ncallr` requires the native to return a value.
- Calling an unknown native is a runtime error.
- Calling a side-effect-only native with `ncallr` is a runtime error.

## Halt

```txt
hlt
```

Stops VM execution immediately. Running past the end of the program also stops
successfully.
