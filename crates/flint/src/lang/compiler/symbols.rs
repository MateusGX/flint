use std::collections::HashMap;

use crate::vm::{Value, MEMORY_SIZE};

use crate::lang::ast;

use super::CompileError;

/// Symbol tables produced by the first compiler pass.
///
/// `labels` holds every `section .text` label: a name starting with `.`
/// (e.g. `.found:`) is *local* to the nearest preceding global label and is
/// stored under the mangled key `{global}.{local}` (e.g. `tasks_get.found`)
/// — this lets the same local label name be reused across different global
/// labels without collision. Global labels (and route handlers, which must
/// be global labels) are stored under their plain name.
///
/// `data` holds every `section .data`/`.bss` label, mapped to its memory
/// address. `initial_memory` is the linear-memory image those declarations
/// produce, indexed by address; it becomes `Program.initial_memory`.
/// `labels` and `data` share one flat namespace — a name can't appear in
/// both.
pub(super) struct Symbols {
    pub(super) labels: HashMap<String, usize>,
    /// `scopes[i]` is the nearest preceding global label for `section .text`
    /// instruction `i`, or `None` if no global label precedes it. Same
    /// length as the `.text` instruction stream (i.e. excludes `data`/`res`
    /// pseudo-instructions, which never produce bytecode).
    pub(super) scopes: Vec<Option<String>>,
    pub(super) data: HashMap<String, i64>,
    pub(super) initial_memory: Vec<Value>,
}

/// Which `section` subsequent items belong to. Defaults to `Text` so files
/// with no `section` directive compile exactly as before.
#[derive(PartialEq)]
enum CurrentSection {
    Text,
    Data,
    Bss,
}

/// A `section .data`/`.bss` label waiting for its `data`/`res`
/// pseudo-instruction.
struct Pending {
    name: String,
    address: usize,
    line: usize,
}

/// First pass: assigns each `section .text` instruction its final index and
/// records where each label points (mangling local `.name` labels against
/// their enclosing global label, and resolving `section .data`/`.bss` labels
/// to memory addresses), without yet looking at instruction operands (a
/// label may be referenced before it's defined, e.g. a backward `jmp loop`).
pub(super) fn collect_symbols(ast: &ast::Program) -> Result<Symbols, CompileError> {
    let mut labels = HashMap::new();
    let mut scopes = Vec::new();
    let mut data = HashMap::new();
    let mut initial_memory = Vec::new();
    let mut index = 0usize;
    let mut current_global: Option<String> = None;
    let mut current_section = CurrentSection::Text;
    let mut pending: Option<Pending> = None;

    fn declare_label(
        labels: &mut HashMap<String, usize>,
        data: &HashMap<String, i64>,
        name: &str,
        index: usize,
        line: usize,
    ) -> Result<(), CompileError> {
        if labels.contains_key(name) || data.contains_key(name) {
            return Err(CompileError {
                line,
                message: format!("label '{name}' is defined more than once"),
            });
        }
        labels.insert(name.to_string(), index);
        Ok(())
    }

    fn declare_data(
        data: &mut HashMap<String, i64>,
        labels: &HashMap<String, usize>,
        name: &str,
        address: usize,
        line: usize,
    ) -> Result<(), CompileError> {
        if data.contains_key(name) || labels.contains_key(name) {
            return Err(CompileError {
                line,
                message: format!("label '{name}' is defined more than once"),
            });
        }
        data.insert(name.to_string(), address as i64);
        Ok(())
    }

    fn push_memory(
        initial_memory: &mut Vec<Value>,
        value: Value,
        line: usize,
    ) -> Result<(), CompileError> {
        initial_memory.push(value);
        if initial_memory.len() > MEMORY_SIZE {
            return Err(CompileError {
                line,
                message: format!(
                    "total .data/.bss size ({}) exceeds memory capacity ({MEMORY_SIZE})",
                    initial_memory.len()
                ),
            });
        }
        Ok(())
    }

    for item in &ast.items {
        match item {
            ast::Item::Section { name, line } => {
                if let Some(pending) = &pending {
                    return Err(CompileError {
                        line: *line,
                        message: format!(
                            "label '{}' has no following data/res directive",
                            pending.name
                        ),
                    });
                }
                use crate::lang::sections;
                current_section = match name.as_str() {
                    s if s == sections::TEXT => CurrentSection::Text,
                    s if s == sections::DATA => CurrentSection::Data,
                    s if s == sections::BSS => CurrentSection::Bss,
                    _ => unreachable!("parser only accepts .text/.data/.bss"),
                };
            }
            ast::Item::Label { name, line } => match current_section {
                CurrentSection::Text => {
                    if let Some(local) = name.strip_prefix('.') {
                        let Some(global) = &current_global else {
                            return Err(CompileError {
                                line: *line,
                                message: format!("local label '{name}' has no enclosing label"),
                            });
                        };
                        declare_label(
                            &mut labels,
                            &data,
                            &format!("{global}.{local}"),
                            index,
                            *line,
                        )?;
                    } else {
                        declare_label(&mut labels, &data, name, index, *line)?;
                        current_global = Some(name.clone());
                    }
                }
                CurrentSection::Data | CurrentSection::Bss => {
                    if name.starts_with('.') {
                        return Err(CompileError {
                            line: *line,
                            message: format!(
                                "local label '{name}' is not valid in section .data/.bss"
                            ),
                        });
                    }
                    if let Some(pending) = &pending {
                        return Err(CompileError {
                            line: *line,
                            message: format!(
                                "label '{}' has no following data/res directive",
                                pending.name
                            ),
                        });
                    }
                    pending = Some(Pending {
                        name: name.clone(),
                        address: initial_memory.len(),
                        line: *line,
                    });
                }
            },
            ast::Item::Route { .. } => {}
            ast::Item::Instruction(instr) => match instr.mnemonic.as_str() {
                "data" => {
                    if current_section != CurrentSection::Data {
                        return Err(CompileError {
                            line: instr.line,
                            message: "'data' is only valid in section .data".to_string(),
                        });
                    }
                    let Some(label) = pending.take() else {
                        return Err(CompileError {
                            line: instr.line,
                            message: "'data' must follow a label".to_string(),
                        });
                    };
                    let value = match instr.operands.as_slice() {
                        [ast::Operand::Imm(v)] => Value::Int(*v),
                        [ast::Operand::Float(v)] => Value::Float(*v),
                        [ast::Operand::Str(s)] => Value::Str(s.as_str().into()),
                        _ => {
                            return Err(CompileError {
                                line: instr.line,
                                message: "'data' expects a constant int, float, or string literal"
                                    .to_string(),
                            })
                        }
                    };
                    push_memory(&mut initial_memory, value, instr.line)?;
                    declare_data(&mut data, &labels, &label.name, label.address, label.line)?;
                }
                "res" => {
                    if current_section != CurrentSection::Bss {
                        return Err(CompileError {
                            line: instr.line,
                            message: "'res' is only valid in section .bss".to_string(),
                        });
                    }
                    let Some(label) = pending.take() else {
                        return Err(CompileError {
                            line: instr.line,
                            message: "'res' must follow a label".to_string(),
                        });
                    };
                    let count = match instr.operands.as_slice() {
                        [ast::Operand::Imm(n)] if *n >= 1 => *n as usize,
                        _ => {
                            return Err(CompileError {
                                line: instr.line,
                                message: "'res' expects a positive integer count".to_string(),
                            })
                        }
                    };
                    for _ in 0..count {
                        push_memory(&mut initial_memory, Value::Int(0), instr.line)?;
                    }
                    declare_data(&mut data, &labels, &label.name, label.address, label.line)?;
                }
                mnemonic => {
                    if current_section != CurrentSection::Text {
                        return Err(CompileError {
                            line: instr.line,
                            message: format!("'{mnemonic}' is not valid in section .data/.bss"),
                        });
                    }
                    scopes.push(current_global.clone());
                    index += 1;
                }
            },
        }
    }

    if let Some(pending) = pending {
        return Err(CompileError {
            line: pending.line,
            message: format!(
                "label '{}' has no following data/res directive",
                pending.name
            ),
        });
    }

    Ok(Symbols {
        labels,
        scopes,
        data,
        initial_memory,
    })
}

#[cfg(test)]
mod tests {
    use crate::vm::{Instr, Program as VmProgram, Value};

    use crate::lang::compiler::{compile, CompileError};
    use crate::lang::lexer::lex;
    use crate::lang::parser::parse;

    fn compile_source(source: &str) -> Result<VmProgram, CompileError> {
        let tokens = lex(source).unwrap();
        let ast = parse(&tokens).unwrap();
        compile(&ast)
    }

    #[test]
    fn reports_duplicate_labels() {
        let err = compile_source("a:\n  hlt\na:\n  hlt\n").unwrap_err();
        assert!(
            err.message.contains("defined more than once"),
            "{}",
            err.message
        );
    }

    #[test]
    fn labels_are_callable_via_call() {
        let program = compile_source("call greet\nhlt\ngreet:\n  ret\n").unwrap();
        assert!(matches!(program.instructions[0], Instr::Call(2)));
    }

    #[test]
    fn local_labels_are_mangled_with_enclosing_global() {
        let program = compile_source("outer:\n  jmp .found\n.found:\n  ret\n").unwrap();
        assert!(matches!(program.instructions[0], Instr::Jmp(1)));
    }

    #[test]
    fn local_label_without_enclosing_label_is_an_error() {
        let err = compile_source(".found:\n  ret\n").unwrap_err();
        assert!(
            err.message
                .contains("local label '.found' has no enclosing label"),
            "{}",
            err.message
        );
    }

    #[test]
    fn duplicate_local_labels_in_different_scopes_are_allowed() {
        let program =
            compile_source("a:\n  jmp .x\n.x:\n  ret\nb:\n  jmp .x\n.x:\n  ret\n").unwrap();
        assert!(matches!(program.instructions[0], Instr::Jmp(1)));
        assert!(matches!(program.instructions[2], Instr::Jmp(3)));
    }

    #[test]
    fn duplicate_local_labels_in_same_scope_are_an_error() {
        let err = compile_source("a:\n.x:\n  ret\n.x:\n  ret\n").unwrap_err();
        assert!(
            err.message.contains("defined more than once"),
            "{}",
            err.message
        );
    }

    #[test]
    fn data_label_resolves_to_memory_address() {
        let program = compile_source(
            "section .data\ncounter:\n    data 41\nsection .text\nmov r0, counter\n",
        )
        .unwrap();
        assert_eq!(program.initial_memory, vec![Value::Int(41)]);
        assert!(matches!(program.instructions[0], Instr::LoadInt(0, 0)));
    }

    #[test]
    fn bss_label_reserves_zeroed_cells() {
        let program =
            compile_source("section .bss\nbuffer:\n    res 4\nsection .text\nmov r0, buffer\n")
                .unwrap();
        assert_eq!(
            program.initial_memory,
            vec![Value::Int(0), Value::Int(0), Value::Int(0), Value::Int(0)]
        );
        assert!(matches!(program.instructions[0], Instr::LoadInt(0, 0)));
    }

    #[test]
    fn multiple_data_and_bss_labels_get_sequential_addresses() {
        let program = compile_source(
            "section .data\na:\n    data 1\nb:\n    data 2\nsection .bss\nc:\n    res 2\nsection .text\nmov r0, a\nmov r1, b\nmov r2, c\n",
        )
        .unwrap();
        assert!(matches!(program.instructions[0], Instr::LoadInt(0, 0)));
        assert!(matches!(program.instructions[1], Instr::LoadInt(1, 1)));
        assert!(matches!(program.instructions[2], Instr::LoadInt(2, 2)));
        assert_eq!(program.initial_memory.len(), 4);
    }

    #[test]
    fn data_outside_data_section_is_an_error() {
        let err = compile_source("a:\n    data 1\n").unwrap_err();
        assert!(
            err.message
                .contains("'data' is only valid in section .data"),
            "{}",
            err.message
        );
    }

    #[test]
    fn res_outside_bss_section_is_an_error() {
        let err = compile_source("section .data\na:\n    res 1\n").unwrap_err();
        assert!(
            err.message.contains("'res' is only valid in section .bss"),
            "{}",
            err.message
        );
    }

    #[test]
    fn label_without_data_or_res_is_an_error() {
        let err = compile_source("section .data\na:\nb:\n    data 1\n").unwrap_err();
        assert!(
            err.message.contains("has no following data/res directive"),
            "{}",
            err.message
        );
    }

    #[test]
    fn data_without_preceding_label_is_an_error() {
        let err = compile_source("section .data\na:\n    data 1\n    data 2\n").unwrap_err();
        assert!(
            err.message.contains("'data' must follow a label"),
            "{}",
            err.message
        );
    }

    #[test]
    fn local_label_in_data_section_is_an_error() {
        let err = compile_source("section .data\n.a:\n    data 1\n").unwrap_err();
        assert!(
            err.message
                .contains("local label '.a' is not valid in section .data/.bss"),
            "{}",
            err.message
        );
    }

    #[test]
    fn instruction_in_data_section_is_an_error() {
        let err = compile_source("section .data\nmov r0, 1\n").unwrap_err();
        assert!(
            err.message
                .contains("'mov' is not valid in section .data/.bss"),
            "{}",
            err.message
        );
    }

    #[test]
    fn data_and_text_labels_share_one_namespace() {
        let err = compile_source("section .data\na:\n    data 1\nsection .text\na:\n  ret\n")
            .unwrap_err();
        assert!(
            err.message.contains("defined more than once"),
            "{}",
            err.message
        );
    }

    #[test]
    fn total_static_memory_exceeding_capacity_is_an_error() {
        let err = compile_source("section .bss\na:\n    res 5000\n").unwrap_err();
        assert!(
            err.message.contains("exceeds memory capacity"),
            "{}",
            err.message
        );
    }
}
