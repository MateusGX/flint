use std::fmt;

/// A lexical token together with the source line it came from (1-based),
/// used for error reporting throughout the rest of the pipeline.
#[derive(Debug, Clone, PartialEq)]
pub struct SpannedToken {
    pub token: Token,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// A bareword: an instruction mnemonic, a register name (`r0`), a label
    /// name, or a native function name (`debug.print`). The parser decides
    /// which based on context and shape.
    Ident(String),
    Int(i64),
    Float(f64),
    Str(String),
    Comma,
    Colon,
    LBracket,
    RBracket,
    /// `->`, used by `route METHOD "/path" -> handler` to point at the
    /// handler function. Lexed greedily before `-` is considered the start
    /// of a negative integer — see the dispatch in `lex`.
    Arrow,
    /// Statement separator — Flint source is line-oriented, like assembly.
    Newline,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LexError {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}: {}", self.line, self.message)
    }
}

impl std::error::Error for LexError {}

/// Tokenizes Flint source text.
///
/// Always appends a trailing [`Token::Newline`] so the parser can treat the
/// final line the same as any other, even if the source doesn't end with one.
pub fn lex(source: &str) -> Result<Vec<SpannedToken>, LexError> {
    let chars: Vec<char> = source.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;
    let mut line = 1usize;

    while i < chars.len() {
        let c = chars[i];
        match c {
            '\n' => {
                tokens.push(SpannedToken {
                    token: Token::Newline,
                    line,
                });
                line += 1;
                i += 1;
            }
            c if c.is_whitespace() => i += 1,
            ';' => {
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
            }
            ',' => {
                tokens.push(SpannedToken {
                    token: Token::Comma,
                    line,
                });
                i += 1;
            }
            ':' => {
                tokens.push(SpannedToken {
                    token: Token::Colon,
                    line,
                });
                i += 1;
            }
            '[' => {
                tokens.push(SpannedToken {
                    token: Token::LBracket,
                    line,
                });
                i += 1;
            }
            ']' => {
                tokens.push(SpannedToken {
                    token: Token::RBracket,
                    line,
                });
                i += 1;
            }
            '"' => {
                let (token, consumed) = lex_string(&chars[i..], line)?;
                tokens.push(SpannedToken { token, line });
                i += consumed;
            }
            '-' if chars.get(i + 1) == Some(&'>') => {
                tokens.push(SpannedToken {
                    token: Token::Arrow,
                    line,
                });
                i += 2;
            }
            c if c == '-' || c.is_ascii_digit() => {
                let (token, consumed) = lex_number(&chars[i..], line)?;
                tokens.push(SpannedToken { token, line });
                i += consumed;
            }
            c if c.is_alphabetic()
                || c == '_'
                || (c == '.'
                    && chars
                        .get(i + 1)
                        .is_some_and(|n| n.is_alphabetic() || *n == '_')) =>
            {
                let start = i;
                while i < chars.len()
                    && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '.')
                {
                    i += 1;
                }
                let text: String = chars[start..i].iter().collect();
                tokens.push(SpannedToken {
                    token: Token::Ident(text),
                    line,
                });
            }
            other => {
                return Err(LexError {
                    line,
                    message: format!("unexpected character '{other}'"),
                })
            }
        }
    }

    tokens.push(SpannedToken {
        token: Token::Newline,
        line,
    });
    Ok(tokens)
}

/// Lexes a quoted string starting at `chars[0] == '"'`. Returns the token and
/// the number of characters consumed (including both quotes).
fn lex_string(chars: &[char], line: usize) -> Result<(Token, usize), LexError> {
    let mut i = 1; // skip the opening quote
    let mut value = String::new();

    loop {
        match chars.get(i) {
            None => {
                return Err(LexError {
                    line,
                    message: "unterminated string literal".to_string(),
                })
            }
            Some('"') => {
                i += 1;
                break;
            }
            Some('\\') => {
                i += 1;
                let escaped = match chars.get(i) {
                    Some('n') => '\n',
                    Some('t') => '\t',
                    Some('"') => '"',
                    Some('\\') => '\\',
                    Some(other) => {
                        return Err(LexError {
                            line,
                            message: format!("unknown escape sequence '\\{other}'"),
                        })
                    }
                    None => {
                        return Err(LexError {
                            line,
                            message: "unterminated escape sequence".to_string(),
                        })
                    }
                };
                value.push(escaped);
                i += 1;
            }
            Some(&ch) => {
                value.push(ch);
                i += 1;
            }
        }
    }

    Ok((Token::Str(value), i))
}

/// Lexes an integer or float literal (optionally negative). Returns the
/// token and the number of characters consumed. A `.` followed by at least
/// one digit after the integer part produces a `Token::Float`.
fn lex_number(chars: &[char], line: usize) -> Result<(Token, usize), LexError> {
    let mut i = 0;
    if chars.first() == Some(&'-') {
        i += 1;
    }
    let digits_start = i;
    while chars.get(i).is_some_and(|c| c.is_ascii_digit()) {
        i += 1;
    }
    if i == digits_start {
        return Err(LexError {
            line,
            message: "expected digits after '-'".to_string(),
        });
    }

    // Check for float: digits '.' digits
    if chars.get(i) == Some(&'.') && chars.get(i + 1).is_some_and(|c| c.is_ascii_digit()) {
        i += 1; // consume '.'
        while chars.get(i).is_some_and(|c| c.is_ascii_digit()) {
            i += 1;
        }
        let text: String = chars[..i].iter().collect();
        let value = text.parse::<f64>().map_err(|_| LexError {
            line,
            message: format!("invalid float literal '{text}'"),
        })?;
        return Ok((Token::Float(value), i));
    }

    let text: String = chars[..i].iter().collect();
    let value = text.parse::<i64>().map_err(|_| LexError {
        line,
        message: format!("invalid integer literal '{text}'"),
    })?;
    Ok((Token::Int(value), i))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token_kinds(tokens: &[SpannedToken]) -> Vec<Token> {
        tokens.iter().map(|t| t.token.clone()).collect()
    }

    #[test]
    fn lexes_a_simple_instruction_line() {
        let tokens = lex("mov r0, 10 ; load the answer\n").unwrap();
        assert_eq!(
            token_kinds(&tokens),
            vec![
                Token::Ident("mov".to_string()),
                Token::Ident("r0".to_string()),
                Token::Comma,
                Token::Int(10),
                Token::Newline,
                Token::Newline,
            ]
        );
    }

    #[test]
    fn lexes_labels_strings_and_memory_operands() {
        let tokens = lex("loop:\n  mov r0, \"hi\\n\"\n  store [r1], r0\n").unwrap();
        assert_eq!(
            token_kinds(&tokens),
            vec![
                Token::Ident("loop".to_string()),
                Token::Colon,
                Token::Newline,
                Token::Ident("mov".to_string()),
                Token::Ident("r0".to_string()),
                Token::Comma,
                Token::Str("hi\n".to_string()),
                Token::Newline,
                Token::Ident("store".to_string()),
                Token::LBracket,
                Token::Ident("r1".to_string()),
                Token::RBracket,
                Token::Comma,
                Token::Ident("r0".to_string()),
                Token::Newline,
                Token::Newline,
            ]
        );
    }

    #[test]
    fn lexes_local_labels_starting_with_a_dot() {
        let tokens = lex(".found:\n  jmp .found\n").unwrap();
        assert_eq!(
            token_kinds(&tokens),
            vec![
                Token::Ident(".found".to_string()),
                Token::Colon,
                Token::Newline,
                Token::Ident("jmp".to_string()),
                Token::Ident(".found".to_string()),
                Token::Newline,
                Token::Newline,
            ]
        );
    }

    #[test]
    fn negative_integers_are_supported() {
        let tokens = lex("mov r0, -5\n").unwrap();
        assert_eq!(tokens[3].token, Token::Int(-5));
    }

    #[test]
    fn arrow_is_distinguished_from_a_negative_integer() {
        let tokens = lex("route GET \"/users\" -> list_users\nmov r0, -5\n").unwrap();
        assert_eq!(
            token_kinds(&tokens),
            vec![
                Token::Ident("route".to_string()),
                Token::Ident("GET".to_string()),
                Token::Str("/users".to_string()),
                Token::Arrow,
                Token::Ident("list_users".to_string()),
                Token::Newline,
                Token::Ident("mov".to_string()),
                Token::Ident("r0".to_string()),
                Token::Comma,
                Token::Int(-5),
                Token::Newline,
                Token::Newline,
            ]
        );
    }

    #[test]
    fn reports_unterminated_string_literal() {
        let err = lex("mov r0, \"oops\n").unwrap_err();
        assert_eq!(err.line, 1);
        assert!(err.message.contains("unterminated string"));
    }

    #[test]
    fn reports_unexpected_characters() {
        let err = lex("mov r0, @\n").unwrap_err();
        assert!(err.message.contains("unexpected character '@'"));
    }
}
