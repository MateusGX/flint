use std::fmt;

use crate::vm::NUM_REGISTERS;

use crate::lang::ast::{Instruction, Item, Operand, Program};
use crate::lang::lexer::{SpannedToken, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}: {}", self.line, self.message)
    }
}

impl std::error::Error for ParseError {}

/// Parses a token stream into a [`Program`] AST.
///
/// Flint is line-oriented (like assembly): each line is either empty, a
/// label definition (`name:`), or an instruction (`mnemonic operand, ...`).
pub fn parse(tokens: &[SpannedToken]) -> Result<Program, ParseError> {
    let mut items = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        while i < tokens.len() && tokens[i].token == Token::Newline {
            i += 1;
        }
        if i >= tokens.len() {
            break;
        }

        let line = tokens[i].line;
        let line_start = i;
        while i < tokens.len() && tokens[i].token != Token::Newline {
            i += 1;
        }
        let line_tokens = &tokens[line_start..i];
        if !line_tokens.is_empty() {
            items.push(parse_line(line_tokens, line)?);
        }
        i += 1; // consume the Newline
    }

    Ok(Program { items })
}

fn parse_line(tokens: &[SpannedToken], line: usize) -> Result<Item, ParseError> {
    if let [SpannedToken {
        token: Token::Ident(name),
        ..
    }, SpannedToken {
        token: Token::Colon,
        ..
    }] = tokens
    {
        return Ok(Item::Label {
            name: name.clone(),
            line,
        });
    }

    if let Some(SpannedToken {
        token: Token::Ident(keyword),
        ..
    }) = tokens.first()
    {
        if keyword == "route" {
            return parse_route(tokens, line);
        }
        if keyword == "section" {
            return parse_section(tokens, line);
        }
    }

    let mnemonic = match &tokens[0].token {
        Token::Ident(name) => name.to_lowercase(),
        other => {
            return Err(ParseError {
                line,
                message: format!(
                    "expected an instruction or label, found {}",
                    describe_token(other)
                ),
            })
        }
    };

    let operands = parse_operand_list(tokens, 1, line)?;
    Ok(Item::Instruction(Instruction {
        mnemonic,
        operands,
        line,
    }))
}

/// `route METHOD "/path" -> handler` — the path must be a quoted string
/// (avoids any ambiguity between `/` as a path separator and as an operator),
/// and `->` points at the handler function's name. Method and handler casing
/// is preserved here; the compiler normalizes and validates the method.
fn parse_route(tokens: &[SpannedToken], line: usize) -> Result<Item, ParseError> {
    match tokens {
        [_, SpannedToken {
            token: Token::Ident(method),
            ..
        }, SpannedToken {
            token: Token::Str(path),
            ..
        }, SpannedToken {
            token: Token::Arrow,
            ..
        }, SpannedToken {
            token: Token::Ident(handler),
            ..
        }] => Ok(Item::Route {
            method: method.clone(),
            path: path.clone(),
            handler: handler.clone(),
            line,
        }),
        _ => Err(ParseError {
            line,
            message: "expected a route directive: 'route METHOD \"/path\" -> handler'".to_string(),
        }),
    }
}

/// `section .text` | `section .data` | `section .bss` — switches which
/// region subsequent labels/instructions belong to.
fn parse_section(tokens: &[SpannedToken], line: usize) -> Result<Item, ParseError> {
    use super::sections;
    match tokens {
        [_, SpannedToken {
            token: Token::Ident(name),
            ..
        }] if sections::COMPILER_SECTIONS.contains(&name.as_str()) => Ok(Item::Section {
            name: name.clone(),
            line,
        }),
        _ => Err(ParseError {
            line,
            message: format!(
                "expected a section directive: 'section {}'",
                sections::COMPILER_SECTIONS.join("', 'section ")
            ),
        }),
    }
}

fn parse_operand_list(
    tokens: &[SpannedToken],
    mut idx: usize,
    line: usize,
) -> Result<Vec<Operand>, ParseError> {
    let mut operands = Vec::new();
    if idx >= tokens.len() {
        return Ok(operands);
    }

    loop {
        let (operand, next) = parse_operand(tokens, idx, line)?;
        operands.push(operand);
        idx = next;
        if idx >= tokens.len() {
            break;
        }
        match &tokens[idx].token {
            Token::Comma => idx += 1,
            other => {
                return Err(ParseError {
                    line,
                    message: format!(
                        "expected ',' between operands, found {}",
                        describe_token(other)
                    ),
                })
            }
        }
    }

    Ok(operands)
}

fn parse_operand(
    tokens: &[SpannedToken],
    idx: usize,
    line: usize,
) -> Result<(Operand, usize), ParseError> {
    match tokens.get(idx).map(|t| &t.token) {
        Some(Token::Int(n)) => Ok((Operand::Imm(*n), idx + 1)),
        Some(Token::Float(f)) => Ok((Operand::Float(*f), idx + 1)),
        Some(Token::Str(s)) => Ok((Operand::Str(s.clone()), idx + 1)),
        Some(Token::LBracket) => {
            let reg = match tokens.get(idx + 1).map(|t| &t.token) {
                Some(Token::Ident(name)) => parse_register(name).ok_or_else(|| ParseError {
                    line,
                    message: format!("expected a register inside '[...]', found '{name}'"),
                })?,
                other => {
                    return Err(ParseError {
                        line,
                        message: format!(
                            "expected a register inside '[...]', found {}",
                            other.map_or("end of line".to_string(), describe_token)
                        ),
                    })
                }
            };
            match tokens.get(idx + 2).map(|t| &t.token) {
                Some(Token::RBracket) => Ok((Operand::Mem(reg), idx + 3)),
                other => Err(ParseError {
                    line,
                    message: format!(
                        "expected ']', found {}",
                        other.map_or("end of line".to_string(), describe_token)
                    ),
                }),
            }
        }
        Some(Token::Ident(name)) => match parse_register(name) {
            Some(reg) => Ok((Operand::Reg(reg), idx + 1)),
            None => Ok((Operand::Ident(name.clone()), idx + 1)),
        },
        other => Err(ParseError {
            line,
            message: format!(
                "expected an operand, found {}",
                other.map_or("end of line".to_string(), describe_token)
            ),
        }),
    }
}

/// Recognizes register names `r0`..`r15`. Anything else shaped like `r<N>`
/// (e.g. `r16`, `rax`) is treated as a plain identifier — the compiler will
/// then report it as an undefined label or an invalid operand, which is
/// still a clear error for the author to act on.
fn parse_register(name: &str) -> Option<u8> {
    let digits = name.strip_prefix('r')?;
    let n: u8 = digits.parse().ok()?;
    if (n as usize) < NUM_REGISTERS {
        Some(n)
    } else {
        None
    }
}

fn describe_token(token: &Token) -> String {
    match token {
        Token::Ident(name) => format!("'{name}'"),
        Token::Int(n) => format!("integer '{n}'"),
        Token::Float(f) => format!("float '{f}'"),
        Token::Str(_) => "a string literal".to_string(),
        Token::Comma => "','".to_string(),
        Token::Colon => "':'".to_string(),
        Token::LBracket => "'['".to_string(),
        Token::RBracket => "']'".to_string(),
        Token::Arrow => "'->'".to_string(),
        Token::Newline => "end of line".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lang::lexer::lex;

    fn parse_source(source: &str) -> Result<Program, ParseError> {
        parse(&lex(source).unwrap())
    }

    #[test]
    fn parses_labels_and_instructions() {
        let program = parse_source("loop:\n  cmp r0, r1\n  jl loop\n").unwrap();
        assert_eq!(
            program.items,
            vec![
                Item::Label {
                    name: "loop".to_string(),
                    line: 1
                },
                Item::Instruction(Instruction {
                    mnemonic: "cmp".to_string(),
                    operands: vec![Operand::Reg(0), Operand::Reg(1)],
                    line: 2,
                }),
                Item::Instruction(Instruction {
                    mnemonic: "jl".to_string(),
                    operands: vec![Operand::Ident("loop".to_string())],
                    line: 3,
                }),
            ]
        );
    }

    #[test]
    fn parses_immediates_strings_and_memory_operands() {
        let program = parse_source("mov r0, \"hi\"\nstore [r1], r0\nload r2, [r1]\n").unwrap();
        assert_eq!(
            program.items,
            vec![
                Item::Instruction(Instruction {
                    mnemonic: "mov".to_string(),
                    operands: vec![Operand::Reg(0), Operand::Str("hi".to_string())],
                    line: 1,
                }),
                Item::Instruction(Instruction {
                    mnemonic: "store".to_string(),
                    operands: vec![Operand::Mem(1), Operand::Reg(0)],
                    line: 2,
                }),
                Item::Instruction(Instruction {
                    mnemonic: "load".to_string(),
                    operands: vec![Operand::Reg(2), Operand::Mem(1)],
                    line: 3,
                }),
            ]
        );
    }

    #[test]
    fn parses_labels_and_route_directives() {
        let program =
            parse_source("list_users:\n  ret\nroute GET \"/users\" -> list_users\n").unwrap();
        assert_eq!(
            program.items,
            vec![
                Item::Label {
                    name: "list_users".to_string(),
                    line: 1
                },
                Item::Instruction(Instruction {
                    mnemonic: "ret".to_string(),
                    operands: vec![],
                    line: 2
                }),
                Item::Route {
                    method: "GET".to_string(),
                    path: "/users".to_string(),
                    handler: "list_users".to_string(),
                    line: 3,
                },
            ]
        );
    }

    #[test]
    fn parses_local_labels_and_references() {
        let program = parse_source("outer:\n  jmp .found\n.found:\n  ret\n").unwrap();
        assert_eq!(
            program.items,
            vec![
                Item::Label {
                    name: "outer".to_string(),
                    line: 1
                },
                Item::Instruction(Instruction {
                    mnemonic: "jmp".to_string(),
                    operands: vec![Operand::Ident(".found".to_string())],
                    line: 2,
                }),
                Item::Label {
                    name: ".found".to_string(),
                    line: 3
                },
                Item::Instruction(Instruction {
                    mnemonic: "ret".to_string(),
                    operands: vec![],
                    line: 4
                }),
            ]
        );
    }

    #[test]
    fn parses_section_directives() {
        let program = parse_source("section .data\nsection .bss\nsection .text\n").unwrap();
        assert_eq!(
            program.items,
            vec![
                Item::Section {
                    name: ".data".to_string(),
                    line: 1
                },
                Item::Section {
                    name: ".bss".to_string(),
                    line: 2
                },
                Item::Section {
                    name: ".text".to_string(),
                    line: 3
                },
            ]
        );
    }

    #[test]
    fn reports_invalid_section_names() {
        let err = parse_source("section .rodata\n").unwrap_err();
        assert!(
            err.message.contains("expected a section directive"),
            "{}",
            err.message
        );
    }

    #[test]
    fn reports_malformed_route_directives() {
        let err = parse_source("route GET \"/users\" handler\n").unwrap_err();
        assert!(
            err.message.contains("expected a route directive"),
            "{}",
            err.message
        );
    }

    #[test]
    fn mnemonics_are_case_insensitive() {
        let program = parse_source("MOV r0, 1\n").unwrap();
        match &program.items[0] {
            Item::Instruction(instr) => assert_eq!(instr.mnemonic, "mov"),
            other => panic!("expected an instruction, found {other:?}"),
        }
    }

    #[test]
    fn reports_missing_comma_between_operands() {
        let err = parse_source("mov r0 r1\n").unwrap_err();
        assert!(
            err.message.contains("expected ',' between operands"),
            "{}",
            err.message
        );
    }

    #[test]
    fn reports_unterminated_memory_operand() {
        let err = parse_source("load r0, [r1\n").unwrap_err();
        assert!(err.message.contains("expected ']'"), "{}", err.message);
    }
}
