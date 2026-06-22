//! Lowering a parsed [`crate::lang::ast::Program`] into VM bytecode — split by pass,
//! one file per responsibility:
//! - [`symbols`] — first pass: records where each label points
//! - [`strings`] — string-literal interning into the constant pool
//! - [`instructions`] — lowering individual `ast::Instruction`s to `Instr`
//! - [`routes`] — `route` directive validation and handler resolution
//!
//! This file wires those passes together behind the two entry points,
//! [`compile`]/[`compile_app`], and holds the shared error/result types.
//! Adding a pass (e.g. an optimization or a new directive kind) means adding
//! a file here and a step in [`compile_app`] — the existing passes don't need
//! to change.

mod instructions;
mod routes;
mod strings;
mod symbols;

use std::fmt;

use crate::vm::Program as VmProgram;

use crate::lang::ast;
use instructions::compile_instruction;
use routes::compile_route;
use strings::StringPool;
use symbols::collect_symbols;

pub use routes::Route;

#[derive(Debug, Clone, PartialEq)]
pub struct CompileError {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}: {}", self.line, self.message)
    }
}

impl std::error::Error for CompileError {}

/// The result of compiling a Flint route source file: bytecode plus the
/// routes it declares. One of these is produced per `.fl` file by
/// [`crate::lang::load_app_dir`].
#[derive(Debug, Clone)]
pub struct CompiledApp {
    pub program: VmProgram,
    pub routes: Vec<Route>,
}

/// Lowers a parsed [`ast::Program`] into VM bytecode: resolves label
/// references to instruction indices and interns string literals into a
/// constant pool that `LoadStr`/`NCall` reference by index.
///
/// Execution always starts at instruction index 0 — there is no separate
/// entry-point directive, so the program's entry code must come first in
/// source order (functions can be defined below it and reached via `call`).
///
/// This is a thin wrapper over [`compile_app`] for the common case of a
/// program with no `route` directives — see that function if you need the
/// routing metadata too.
pub fn compile(ast: &ast::Program) -> Result<VmProgram, CompileError> {
    Ok(compile_app(ast)?.program)
}

/// Like [`compile`], but also extracts `route` directives into routing
/// metadata, resolving each handler name against the program's function
/// table. This is what [`crate::lang::load_app_dir`] uses to compile each file of
/// an HTTP app.
pub fn compile_app(ast: &ast::Program) -> Result<CompiledApp, CompileError> {
    let symbols = collect_symbols(ast)?;
    let mut strings = StringPool::default();
    let mut instructions = Vec::new();
    let mut routes = Vec::new();

    for item in &ast.items {
        match item {
            ast::Item::Instruction(instr) => {
                if instr.mnemonic == "data" || instr.mnemonic == "res" {
                    continue;
                }
                let scope = symbols.scopes[instructions.len()].as_deref();
                instructions.push(compile_instruction(
                    instr,
                    &symbols.labels,
                    &symbols.data,
                    scope,
                    &mut strings,
                )?);
            }
            ast::Item::Route {
                method,
                path,
                handler,
                line,
            } => {
                routes.push(compile_route(method, path, handler, *line, &symbols)?);
            }
            ast::Item::Label { .. } | ast::Item::Section { .. } => {}
        }
    }

    Ok(CompiledApp {
        program: VmProgram {
            instructions,
            strings: strings.into_vec(),
            initial_memory: symbols.initial_memory,
        },
        routes,
    })
}

#[cfg(test)]
mod tests {
    use crate::vm::Instr;

    use super::*;
    use crate::lang::lexer::lex;
    use crate::lang::parser::parse;

    fn compile_source(source: &str) -> Result<VmProgram, CompileError> {
        let tokens = lex(source).unwrap();
        let ast = parse(&tokens).unwrap();
        compile(&ast)
    }

    #[test]
    fn resolves_forward_and_backward_label_references() {
        let program = compile_source("jmp skip\nloop:\n  jmp loop\nskip:\n  hlt\n").unwrap();
        assert!(matches!(program.instructions[0], Instr::Jmp(2)));
        assert!(matches!(program.instructions[1], Instr::Jmp(1)));
        assert!(matches!(program.instructions[2], Instr::Hlt));
    }
}
