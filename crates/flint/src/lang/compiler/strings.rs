use std::collections::HashMap;
use std::sync::Arc;

/// Interns string literals into a flat constant pool: `LoadStr`/`NCall`
/// reference strings by index, so a duplicate literal (e.g. the same route
/// path or native name appearing twice) reuses the same slot.
#[derive(Default)]
pub(super) struct StringPool {
    strings: Vec<Arc<str>>,
    indices: HashMap<String, u32>,
}

impl StringPool {
    pub(super) fn intern(&mut self, s: &str) -> u32 {
        if let Some(&idx) = self.indices.get(s) {
            return idx;
        }
        let idx = self.strings.len() as u32;
        self.strings.push(Arc::from(s));
        self.indices.insert(s.to_string(), idx);
        idx
    }

    pub(super) fn into_vec(self) -> Vec<Arc<str>> {
        self.strings
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
    fn interns_duplicate_string_constants() {
        let program = compile_source("mov r0, \"hi\"\nmov r1, \"hi\"\nmov r2, \"bye\"\n").unwrap();
        assert_eq!(program.strings.len(), 2);
        assert!(matches!(program.instructions[0], Instr::LoadStr(0, 0)));
        assert!(matches!(program.instructions[1], Instr::LoadStr(1, 0)));
        assert!(matches!(program.instructions[2], Instr::LoadStr(2, 1)));
    }
}
