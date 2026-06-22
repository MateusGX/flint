//! Lexer, parser and bytecode compiler for Flint, the assembly-like
//! language executed by `crate::vm`.

pub mod app;
pub mod ast;
pub mod compiler;
pub mod error;
pub mod lexer;
pub mod pages;
pub mod parser;
pub mod preprocessor;
pub(crate) mod sections;

pub use app::{load_app_dir, AppModule, LoadError};
pub use compiler::{CompiledApp, Route};
pub use error::Error;
pub use pages::{compile_page_source, load_pages_dir, CompiledPageSource, PageCompileError};
pub use preprocessor::{expand as expand_source, ExpandError};

/// Compiles Flint source into a VM-ready [`crate::vm::Program`].
/// For source that declares HTTP routes, use [`compile_app_source`] instead.
pub fn compile_source(source: &str) -> Result<crate::vm::Program, Error> {
    let tokens = lexer::lex(source)?;
    let ast = parser::parse(&tokens)?;
    Ok(compiler::compile(&ast)?)
}

/// Compiles Flint app source — source with `section .route` declarations —
/// into a [`CompiledApp`] with bytecode and resolved route addresses.
///
/// Route declarations **must** use the section format:
/// ```text
/// section .route
///     GET "/hello" -> say_hello
///
/// section .text
/// say_hello:
///     mov r0, "Hello!"
///     ncall http.text, r0
///     ret
/// ```
pub fn compile_app_source(source: &str) -> Result<CompiledApp, Error> {
    preprocessor::validate_sections(source).map_err(|msg| {
        Error::Compile(compiler::CompileError {
            line: 1,
            message: msg,
        })
    })?;
    let normalized = preprocessor::normalize_sections(source);
    compile_app_source_raw(&normalized)
}

/// Compiles already-normalized source without section validation.
/// Used by the file-loading pipeline (after validate → normalize → expand)
/// and by [`CompiledPageSource::compile`] for machine-generated page source.
pub(crate) fn compile_app_source_raw(source: &str) -> Result<CompiledApp, Error> {
    let tokens = lexer::lex(source)?;
    let ast = parser::parse(&tokens)?;
    Ok(compiler::compile_app(&ast)?)
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
