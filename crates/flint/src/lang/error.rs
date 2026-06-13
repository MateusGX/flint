use std::fmt;

use crate::lang::compiler::CompileError;
use crate::lang::lexer::LexError;
use crate::lang::parser::ParseError;

/// Unified error for the whole `source -> bytecode` pipeline, so callers of
/// [`crate::lang::compile_source`] can handle failures from any stage uniformly
/// while still being able to match on the originating stage if they need to.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Lex(LexError),
    Parse(ParseError),
    Compile(CompileError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Lex(e) => write!(f, "syntax error: {e}"),
            Error::Parse(e) => write!(f, "syntax error: {e}"),
            Error::Compile(e) => write!(f, "compile error: {e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<LexError> for Error {
    fn from(e: LexError) -> Self {
        Error::Lex(e)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::Parse(e)
    }
}

impl From<CompileError> for Error {
    fn from(e: CompileError) -> Self {
        Error::Compile(e)
    }
}
