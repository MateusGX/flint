//! Lexer, parser and bytecode compiler for Flint, the assembly-like
//! language executed by `crate::vm`.
//!
//! [`compile_source`] is the single entry point most callers need: it runs
//! source text through lexing, parsing and compilation and produces an
//! [`crate::vm::Program`] ready to hand to [`crate::vm::Vm::run`].

pub mod app;
pub mod ast;
pub mod compiler;
pub mod error;
pub mod lexer;
pub mod pages;
pub mod parser;
pub mod preprocessor;

pub use app::{load_app_dir, AppModule, LoadError};
pub use compiler::{CompiledApp, Route};
pub use error::Error;
pub use pages::{compile_page_source, load_pages_dir, CompiledPageSource, PageCompileError};
pub use preprocessor::{expand as expand_source, ExpandError};

/// Compiles Flint source text into a VM-ready [`crate::vm::Program`].
///
/// This is what you want for plain programs — including ones that define
/// `fn` handlers — as long as you don't need their `route` metadata. For
/// that (e.g. when loading an HTTP app), see [`compile_app_source`].
pub fn compile_source(source: &str) -> Result<crate::vm::Program, Error> {
    let tokens = lexer::lex(source)?;
    let ast = parser::parse(&tokens)?;
    let program = compiler::compile(&ast)?;
    Ok(program)
}

/// Compiles Flint "app" source — source that may declare `route` directives
/// — into a [`CompiledApp`]: bytecode plus the routes it declares, with each
/// handler resolved to a concrete address. [`load_app_dir`] is the usual way
/// to reach this; call it directly only if you're compiling app source from
/// somewhere other than a directory of files.
pub fn compile_app_source(source: &str) -> Result<CompiledApp, Error> {
    let tokens = lexer::lex(source)?;
    let ast = parser::parse(&tokens)?;
    let app = compiler::compile_app(&ast)?;
    Ok(app)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiles_a_small_program_end_to_end() {
        let program = compile_source("mov r0, 1\nmov r1, 2\nadd r2, r0, r1\nhlt\n").unwrap();
        assert_eq!(program.instructions.len(), 4);
    }

    #[test]
    fn surfaces_lex_errors_through_the_unified_error_type() {
        let err = compile_source("mov r0, \"unterminated\n").unwrap_err();
        assert!(matches!(err, Error::Lex(_)));
    }

    #[test]
    fn surfaces_compile_errors_through_the_unified_error_type() {
        let err = compile_source("jmp nowhere\n").unwrap_err();
        assert!(matches!(err, Error::Compile(_)));
    }
}
