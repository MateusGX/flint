use std::collections::HashMap;

use crate::vm::{Instr, Reg};

use crate::lang::ast::{self, Operand};

use super::strings::StringPool;
use super::CompileError;

pub(super) fn compile_instruction(
    instr: &ast::Instruction,
    labels: &HashMap<String, usize>,
    data: &HashMap<String, i64>,
    scope: Option<&str>,
    strings: &mut StringPool,
) -> Result<Instr, CompileError> {
    let line = instr.line;
    let mnemonic = instr.mnemonic.as_str();
    let ops = &instr.operands;

    let err = |message: String| CompileError { line, message };

    let arity = |expected: usize| -> Result<(), CompileError> {
        if ops.len() == expected {
            Ok(())
        } else {
            Err(err(format!(
                "'{mnemonic}' expects {expected} operand(s), found {}",
                ops.len()
            )))
        }
    };
    let reg = |idx: usize| -> Result<Reg, CompileError> {
        match ops.get(idx) {
            Some(Operand::Reg(r)) => Ok(*r),
            Some(other) => Err(err(format!(
                "'{mnemonic}' expects a register as operand {}, found {}",
                idx + 1,
                describe(other)
            ))),
            None => Err(err(format!("'{mnemonic}' is missing operand {}", idx + 1))),
        }
    };
    let label = |idx: usize| -> Result<usize, CompileError> {
        match ops.get(idx) {
            Some(Operand::Ident(name)) => {
                let resolved = match name.strip_prefix('.') {
                    Some(local) => match scope {
                        Some(global) => format!("{global}.{local}"),
                        None => {
                            return Err(err(format!("local label '{name}' has no enclosing label")))
                        }
                    },
                    None => name.clone(),
                };
                labels
                    .get(&resolved)
                    .copied()
                    .ok_or_else(|| err(format!("undefined label '{name}'")))
            }
            Some(other) => Err(err(format!(
                "'{mnemonic}' expects a label as operand {}, found {}",
                idx + 1,
                describe(other)
            ))),
            None => Err(err(format!("'{mnemonic}' is missing operand {}", idx + 1))),
        }
    };

    match mnemonic {
        "mov" => {
            arity(2)?;
            let dst = reg(0)?;
            match &ops[1] {
                Operand::Reg(src) => Ok(Instr::Mov(dst, *src)),
                Operand::Imm(value) => Ok(Instr::LoadInt(dst, *value)),
                Operand::Float(f) => Ok(Instr::LoadFloat(dst, *f)),
                Operand::Str(s) => Ok(Instr::LoadStr(dst, strings.intern(s))),
                Operand::Ident(name) => match data.get(name) {
                    Some(&addr) => Ok(Instr::LoadInt(dst, addr)),
                    None => Err(err(format!(
                        "'mov' cannot load a value from undefined label '{name}' — expected a register, integer, float, string, or a 'section .data'/'.bss' label"
                    ))),
                },
                other => Err(err(format!(
                    "'mov' cannot load a value from {} — expected a register, integer, float or string",
                    describe(other)
                ))),
            }
        }
        "add" => compile_arith(ops, line, mnemonic, &reg, Instr::Add, Instr::AddImm),
        "sub" => compile_arith(ops, line, mnemonic, &reg, Instr::Sub, Instr::SubImm),
        "mul" => compile_arith(ops, line, mnemonic, &reg, Instr::Mul, Instr::MulImm),
        "div" => compile_arith(ops, line, mnemonic, &reg, Instr::Div, Instr::DivImm),
        "mod" => compile_arith(ops, line, mnemonic, &reg, Instr::Mod, Instr::ModImm),
        "addf" => compile_reg3(ops, line, mnemonic, &reg, Instr::AddF),
        "subf" => compile_reg3(ops, line, mnemonic, &reg, Instr::SubF),
        "mulf" => compile_reg3(ops, line, mnemonic, &reg, Instr::MulF),
        "divf" => compile_reg3(ops, line, mnemonic, &reg, Instr::DivF),
        "and" => compile_arith(ops, line, mnemonic, &reg, Instr::And, Instr::AndImm),
        "or" => compile_arith(ops, line, mnemonic, &reg, Instr::Or, Instr::OrImm),
        "xor" => compile_arith(ops, line, mnemonic, &reg, Instr::Xor, Instr::XorImm),
        "not" => compile_reg2(ops, line, mnemonic, &reg, Instr::Not),
        "shl" => compile_arith(ops, line, mnemonic, &reg, Instr::Shl, Instr::ShlImm),
        "shr" => compile_arith(ops, line, mnemonic, &reg, Instr::Shr, Instr::ShrImm),
        "neg" => compile_reg2(ops, line, mnemonic, &reg, Instr::Neg),
        "typeof" => compile_reg2(ops, line, mnemonic, &reg, Instr::TypeOf),
        "cmp" => {
            arity(2)?;
            let a = reg(0)?;
            match ops.get(1) {
                Some(Operand::Reg(b)) => Ok(Instr::Cmp(a, *b)),
                Some(Operand::Imm(value)) => Ok(Instr::CmpImm(a, *value)),
                Some(other) => Err(err(format!(
                    "'cmp' expects a register or integer as operand 2, found {}",
                    describe(other)
                ))),
                None => Err(err("'cmp' is missing operand 2".to_string())),
            }
        }
        "jmp" => {
            arity(1)?;
            Ok(Instr::Jmp(label(0)?))
        }
        "je" => {
            arity(1)?;
            Ok(Instr::Je(label(0)?))
        }
        "jne" => {
            arity(1)?;
            Ok(Instr::Jne(label(0)?))
        }
        "jl" => {
            arity(1)?;
            Ok(Instr::Jl(label(0)?))
        }
        "jg" => {
            arity(1)?;
            Ok(Instr::Jg(label(0)?))
        }
        "jle" => {
            arity(1)?;
            Ok(Instr::Jle(label(0)?))
        }
        "jge" => {
            arity(1)?;
            Ok(Instr::Jge(label(0)?))
        }
        "push" => {
            arity(1)?;
            Ok(Instr::Push(reg(0)?))
        }
        "pop" => {
            arity(1)?;
            Ok(Instr::Pop(reg(0)?))
        }
        "call" => {
            arity(1)?;
            Ok(Instr::Call(label(0)?))
        }
        "ret" => {
            arity(0)?;
            Ok(Instr::Ret)
        }
        "hlt" => {
            arity(0)?;
            Ok(Instr::Hlt)
        }
        "load" => {
            arity(2)?;
            let dst = reg(0)?;
            match ops.get(1) {
                Some(Operand::Mem(addr)) => Ok(Instr::Load(dst, *addr)),
                Some(other) => Err(err(format!(
                    "'load' expects a memory operand like '[r1]' as operand 2, found {}",
                    describe(other)
                ))),
                None => Err(err("'load' is missing its source operand".to_string())),
            }
        }
        "store" => {
            arity(2)?;
            match (ops.first(), ops.get(1)) {
                (Some(Operand::Mem(addr)), Some(Operand::Reg(src))) => {
                    Ok(Instr::Store(*addr, *src))
                }
                _ => Err(err("'store' expects '[r<addr>], r<src>'".to_string())),
            }
        }
        "ncall" | "ncallr" => compile_ncall(mnemonic, ops, line, strings),
        other => Err(err(format!("unknown instruction '{other}'"))),
    }
}

/// Compiles a 3-operand arithmetic instruction (`op dst, a, b`), picking the
/// register or register-immediate opcode form based on the shape of the
/// third operand — mirroring how real assemblers select an encoding from the
/// operands rather than requiring the author to spell out which form to use.
fn compile_arith(
    ops: &[Operand],
    line: usize,
    mnemonic: &str,
    reg: &impl Fn(usize) -> Result<Reg, CompileError>,
    reg_form: fn(Reg, Reg, Reg) -> Instr,
    imm_form: fn(Reg, Reg, i64) -> Instr,
) -> Result<Instr, CompileError> {
    let err = |message: String| CompileError { line, message };
    if ops.len() != 3 {
        return Err(err(format!(
            "'{mnemonic}' expects 3 operand(s), found {}",
            ops.len()
        )));
    }
    let dst = reg(0)?;
    let a = reg(1)?;
    match ops.get(2) {
        Some(Operand::Reg(b)) => Ok(reg_form(dst, a, *b)),
        Some(Operand::Imm(value)) => Ok(imm_form(dst, a, *value)),
        Some(other) => Err(err(format!(
            "'{mnemonic}' expects a register or integer as operand 3, found {}",
            describe(other)
        ))),
        None => Err(err(format!("'{mnemonic}' is missing operand 3"))),
    }
}

/// `ncall name, arg, ...`        — calls a native, discarding any result
/// `ncallr dst, name, arg, ...`  — calls a native, storing its result in `dst`
fn compile_ncall(
    mnemonic: &str,
    ops: &[Operand],
    line: usize,
    strings: &mut StringPool,
) -> Result<Instr, CompileError> {
    let err = |message: String| CompileError { line, message };
    let with_dst = mnemonic == "ncallr";

    let mut idx = 0;
    let dst = if with_dst {
        match ops.first() {
            Some(Operand::Reg(r)) => {
                idx = 1;
                Some(*r)
            }
            Some(other) => {
                return Err(err(format!(
                    "'ncallr' expects a destination register as operand 1, found {}",
                    describe(other)
                )))
            }
            None => {
                return Err(err(
                    "'ncallr' is missing its destination register".to_string()
                ))
            }
        }
    } else {
        None
    };

    let name = match ops.get(idx) {
        Some(Operand::Ident(name)) => name.clone(),
        Some(other) => {
            return Err(err(format!(
                "'{mnemonic}' expects a native function name, found {}",
                describe(other)
            )))
        }
        None => {
            return Err(err(format!(
                "'{mnemonic}' is missing the native function name"
            )))
        }
    };
    idx += 1;

    let mut args = Vec::with_capacity(ops.len().saturating_sub(idx));
    for operand in &ops[idx..] {
        match operand {
            Operand::Reg(r) => args.push(*r),
            other => {
                return Err(err(format!(
                    "'{mnemonic}' arguments must be registers, found {}",
                    describe(other)
                )))
            }
        }
    }

    Ok(Instr::NCall {
        name_idx: strings.intern(&name),
        args,
        dst,
    })
}

/// 3-register instruction (no immediate form): `op dst, a, b`.
fn compile_reg3(
    ops: &[Operand],
    line: usize,
    mnemonic: &str,
    reg: &impl Fn(usize) -> Result<Reg, CompileError>,
    make: fn(Reg, Reg, Reg) -> Instr,
) -> Result<Instr, CompileError> {
    let err = |message: String| CompileError { line, message };
    if ops.len() != 3 {
        return Err(err(format!(
            "'{mnemonic}' expects 3 operands, found {}",
            ops.len()
        )));
    }
    Ok(make(reg(0)?, reg(1)?, reg(2)?))
}

/// 2-register instruction: `op dst, src`.
fn compile_reg2(
    ops: &[Operand],
    line: usize,
    mnemonic: &str,
    reg: &impl Fn(usize) -> Result<Reg, CompileError>,
    make: fn(Reg, Reg) -> Instr,
) -> Result<Instr, CompileError> {
    let err = |message: String| CompileError { line, message };
    if ops.len() != 2 {
        return Err(err(format!(
            "'{mnemonic}' expects 2 operands, found {}",
            ops.len()
        )));
    }
    Ok(make(reg(0)?, reg(1)?))
}

fn describe(operand: &Operand) -> String {
    match operand {
        Operand::Reg(r) => format!("register r{r}"),
        Operand::Imm(n) => format!("integer {n}"),
        Operand::Float(f) => format!("float {f}"),
        Operand::Str(_) => "a string literal".to_string(),
        Operand::Mem(r) => format!("memory operand [r{r}]"),
        Operand::Ident(name) => format!("identifier '{name}'"),
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::{Instr, Program as VmProgram};

    use crate::lang::compiler::{compile, CompileError};
    use crate::lang::lexer::lex;
    use crate::lang::parser::parse;

    fn compile_source(source: &str) -> Result<VmProgram, CompileError> {
        let tokens = lex(source).unwrap();
        let ast = parse(&tokens).unwrap();
        compile(&ast)
    }

    #[test]
    fn compiles_ncall_and_ncallr_with_argument_registers() {
        let program =
            compile_source("ncall debug.print, r0, r1\nncallr r2, debug.read, r3\n").unwrap();
        match &program.instructions[0] {
            Instr::NCall { args, dst, .. } => {
                assert_eq!(args, &vec![0, 1]);
                assert_eq!(*dst, None);
            }
            other => panic!("expected NCall, found {other:?}"),
        }
        match &program.instructions[1] {
            Instr::NCall { args, dst, .. } => {
                assert_eq!(args, &vec![3]);
                assert_eq!(*dst, Some(2));
            }
            other => panic!("expected NCall, found {other:?}"),
        }
    }

    #[test]
    fn reports_undefined_labels() {
        let err = compile_source("jmp nowhere\n").unwrap_err();
        assert!(
            err.message.contains("undefined label 'nowhere'"),
            "{}",
            err.message
        );
    }

    #[test]
    fn resolves_local_label_within_scope() {
        let program = compile_source("outer:\n  jmp .found\n.found:\n  ret\n").unwrap();
        assert!(matches!(program.instructions[0], Instr::Jmp(1)));
    }

    #[test]
    fn reports_undefined_local_label_outside_its_scope() {
        let err = compile_source("a:\n.x:\n  ret\nb:\n  jmp .x\n").unwrap_err();
        assert!(
            err.message.contains("undefined label '.x'"),
            "{}",
            err.message
        );
    }

    #[test]
    fn reports_local_label_reference_with_no_scope() {
        let err = compile_source("jmp .found\nouter:\n.found:\n  ret\n").unwrap_err();
        assert!(
            err.message
                .contains("local label '.found' has no enclosing label"),
            "{}",
            err.message
        );
    }

    #[test]
    fn reports_wrong_operand_arity() {
        let err = compile_source("add r0, r1\n").unwrap_err();
        assert!(
            err.message.contains("'add' expects 3 operand(s)"),
            "{}",
            err.message
        );
    }

    #[test]
    fn cmp_and_arithmetic_accept_immediate_right_hand_operands() {
        let program = compile_source("cmp r0, 5\nadd r1, r1, 1\n").unwrap();
        assert!(matches!(program.instructions[0], Instr::CmpImm(0, 5)));
        assert!(matches!(program.instructions[1], Instr::AddImm(1, 1, 1)));
    }

    #[test]
    fn reports_wrong_operand_type() {
        let err = compile_source("cmp r0, \"oops\"\n").unwrap_err();
        assert!(
            err.message.contains("expects a register or integer"),
            "{}",
            err.message
        );
    }

    #[test]
    fn reports_unknown_instruction() {
        let err = compile_source("frobnicate r0\n").unwrap_err();
        assert!(
            err.message.contains("unknown instruction 'frobnicate'"),
            "{}",
            err.message
        );
    }

    #[test]
    fn mov_from_data_label_loads_its_address() {
        let program = compile_source(
            "section .data\ncounter:\n    data 41\nsection .text\nmov r0, counter\n",
        )
        .unwrap();
        assert!(matches!(program.instructions[0], Instr::LoadInt(0, 0)));
    }

    #[test]
    fn mov_from_undefined_ident_is_an_error() {
        let err = compile_source("mov r0, nowhere\n").unwrap_err();
        assert!(
            err.message
                .contains("cannot load a value from undefined label 'nowhere'"),
            "{}",
            err.message
        );
    }
}
