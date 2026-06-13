// Transcribed from docs/reference/instructions.md — keep in sync when
// instructions are added, removed, or documented differently.

export interface InstructionDoc {
  /** Mnemonic, e.g. "mov". */
  name: string;
  /** Snippet body inserted on completion, with operand placeholders. */
  snippet: string;
  /** Canonical operand shape, e.g. "mov dst, src". */
  signature: string;
  /** Short effect description. */
  doc: string;
}

export const INSTRUCTIONS: InstructionDoc[] = [
  {
    name: "mov",
    snippet: "mov ${1:dst}, ${2:src}",
    signature: "mov dst, src | mov dst, int | mov dst, float | mov dst, \"str\"",
    doc: "Copies a register, integer literal, float literal, or string literal into dst.",
  },
  {
    name: "add",
    snippet: "add ${1:dst}, ${2:a}, ${3:b}",
    signature: "add dst, a, b",
    doc: "dst = a + b. Checked integer addition; fails on overflow.",
  },
  {
    name: "sub",
    snippet: "sub ${1:dst}, ${2:a}, ${3:b}",
    signature: "sub dst, a, b",
    doc: "dst = a - b. Checked integer subtraction; fails on overflow.",
  },
  {
    name: "mul",
    snippet: "mul ${1:dst}, ${2:a}, ${3:b}",
    signature: "mul dst, a, b",
    doc: "dst = a * b. Checked integer multiplication; fails on overflow.",
  },
  {
    name: "div",
    snippet: "div ${1:dst}, ${2:a}, ${3:b}",
    signature: "div dst, a, b",
    doc: "Integer division. Fails on division by zero or overflow.",
  },
  {
    name: "mod",
    snippet: "mod ${1:dst}, ${2:a}, ${3:b}",
    signature: "mod dst, a, b",
    doc: "Integer remainder. Fails on division by zero or overflow.",
  },
  {
    name: "addf",
    snippet: "addf ${1:dst}, ${2:a}, ${3:b}",
    signature: "addf dst, a, b",
    doc: "dst = a + b. Operands may be int or float; integers are promoted.",
  },
  {
    name: "subf",
    snippet: "subf ${1:dst}, ${2:a}, ${3:b}",
    signature: "subf dst, a, b",
    doc: "dst = a - b. Operands may be int or float; integers are promoted.",
  },
  {
    name: "mulf",
    snippet: "mulf ${1:dst}, ${2:a}, ${3:b}",
    signature: "mulf dst, a, b",
    doc: "dst = a * b. Operands may be int or float; integers are promoted.",
  },
  {
    name: "divf",
    snippet: "divf ${1:dst}, ${2:a}, ${3:b}",
    signature: "divf dst, a, b",
    doc: "dst = a / b. Fails when b is 0.0.",
  },
  {
    name: "and",
    snippet: "and ${1:dst}, ${2:a}, ${3:b}",
    signature: "and dst, a, b",
    doc: "Bitwise AND. Right operand may be a register or integer literal.",
  },
  {
    name: "or",
    snippet: "or ${1:dst}, ${2:a}, ${3:b}",
    signature: "or dst, a, b",
    doc: "Bitwise OR. Right operand may be a register or integer literal.",
  },
  {
    name: "xor",
    snippet: "xor ${1:dst}, ${2:a}, ${3:b}",
    signature: "xor dst, a, b",
    doc: "Bitwise XOR. Right operand may be a register or integer literal.",
  },
  {
    name: "not",
    snippet: "not ${1:dst}, ${2:src}",
    signature: "not dst, src",
    doc: "Bitwise NOT.",
  },
  {
    name: "shl",
    snippet: "shl ${1:dst}, ${2:a}, ${3:b}",
    signature: "shl dst, a, b",
    doc: "Shift left. Shift amount must be in 0..63.",
  },
  {
    name: "shr",
    snippet: "shr ${1:dst}, ${2:a}, ${3:b}",
    signature: "shr dst, a, b",
    doc: "Arithmetic shift right. Shift amount must be in 0..63.",
  },
  {
    name: "neg",
    snippet: "neg ${1:dst}, ${2:src}",
    signature: "neg dst, src",
    doc: "Negates an int or float. Fails on integer overflow and non-numeric values.",
  },
  {
    name: "typeof",
    snippet: "typeof ${1:dst}, ${2:src}",
    signature: "typeof dst, src",
    doc: 'Stores "int", "float", "str", or "json" in dst.',
  },
  {
    name: "cmp",
    snippet: "cmp ${1:a}, ${2:b}",
    signature: "cmp a, b | cmp a, imm",
    doc: "Compares integers and updates VM flags for conditional jumps.",
  },
  {
    name: "jmp",
    snippet: "jmp ${1:target}",
    signature: "jmp target",
    doc: "Always jumps to target.",
  },
  {
    name: "je",
    snippet: "je ${1:target}",
    signature: "je target",
    doc: "Jumps to target when the last cmp was equal.",
  },
  {
    name: "jne",
    snippet: "jne ${1:target}",
    signature: "jne target",
    doc: "Jumps to target when the last cmp was not equal.",
  },
  {
    name: "jl",
    snippet: "jl ${1:target}",
    signature: "jl target",
    doc: "Jumps to target when the last cmp was less than.",
  },
  {
    name: "jg",
    snippet: "jg ${1:target}",
    signature: "jg target",
    doc: "Jumps to target when the last cmp was greater than.",
  },
  {
    name: "jle",
    snippet: "jle ${1:target}",
    signature: "jle target",
    doc: "Jumps to target when the last cmp was less than or equal.",
  },
  {
    name: "jge",
    snippet: "jge ${1:target}",
    signature: "jge target",
    doc: "Jumps to target when the last cmp was greater than or equal.",
  },
  {
    name: "push",
    snippet: "push ${1:src}",
    signature: "push src",
    doc: "Pushes a register value onto the VM stack.",
  },
  {
    name: "pop",
    snippet: "pop ${1:dst}",
    signature: "pop dst",
    doc: "Pops the top stack value into dst. Fails on stack underflow.",
  },
  {
    name: "call",
    snippet: "call ${1:target}",
    signature: "call target",
    doc: "Pushes a return address and jumps to a label or function. Call depth limit is 1024.",
  },
  {
    name: "ret",
    snippet: "ret",
    signature: "ret",
    doc: "Pops the return address and jumps back.",
  },
  {
    name: "hlt",
    snippet: "hlt",
    signature: "hlt",
    doc: "Stops VM execution immediately.",
  },
  {
    name: "load",
    snippet: "load ${1:dst}, [${2:addr}]",
    signature: "load dst, [addr]",
    doc: "Loads memory[register addr] into dst.",
  },
  {
    name: "store",
    snippet: "store [${1:addr}], ${2:src}",
    signature: "store [addr], src",
    doc: "Stores src into memory[register addr].",
  },
  {
    name: "data",
    snippet: "data ${1:value}",
    signature: "data <value>",
    doc: "Reserves one memory cell initialized to an int, float, or string literal. Only valid in section .data, immediately after a label.",
  },
  {
    name: "res",
    snippet: "res ${1:count}",
    signature: "res <count>",
    doc: "Reserves <count> zero-initialized memory cells. Only valid in section .bss, immediately after a label.",
  },
  {
    name: "ncall",
    snippet: "ncall ${1:native}, ${2:arg}",
    signature: "ncall native, arg1, arg2, ...",
    doc: "Calls a native function for its side effects. Arguments must be registers.",
  },
  {
    name: "ncallr",
    snippet: "ncallr ${1:dst}, ${2:native}, ${3:arg}",
    signature: "ncallr dst, native, arg1, arg2, ...",
    doc: "Calls a native function and stores its return value in dst. Arguments must be registers.",
  },
];
