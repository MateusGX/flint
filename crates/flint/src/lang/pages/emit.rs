use std::collections::HashMap;

use super::compiler::{HTML, S1, S2, S3, S4, S5};
use super::source::flint_string_literal;
use super::PageCompileError;

const SCRATCH_REGISTERS: [&str; 5] = [S1, S2, S3, S4, S5];

pub(super) fn emit0(out: &mut String, native: &str) {
    emit_call(out, native, &[]);
}

pub(super) fn emit1(out: &mut String, native: &str, a: &str, data: &HashMap<String, String>) {
    emit_args(out, native, &[a], data);
}

pub(super) fn emit2(
    out: &mut String,
    native: &str,
    a1: &str,
    a2: &str,
    data: &HashMap<String, String>,
) {
    emit_args(out, native, &[a1, a2], data);
}

pub(super) fn emit3(
    out: &mut String,
    native: &str,
    a1: &str,
    a2: &str,
    a3: &str,
    data: &HashMap<String, String>,
) {
    emit_args(out, native, &[a1, a2, a3], data);
}

pub(super) fn emit4(
    out: &mut String,
    native: &str,
    a1: &str,
    a2: &str,
    a3: &str,
    a4: &str,
    data: &HashMap<String, String>,
) {
    emit_args(out, native, &[a1, a2, a3, a4], data);
}

pub(super) fn emit5(
    out: &mut String,
    native: &str,
    args: [&str; 5],
    data: &HashMap<String, String>,
) {
    emit_args(out, native, &args, data);
}

fn emit_args(out: &mut String, native: &str, args: &[&str], data: &HashMap<String, String>) {
    let owned: Vec<String> = args
        .iter()
        .zip(SCRATCH_REGISTERS)
        .map(|(arg, scratch)| load_arg(arg, data, scratch, out))
        .collect();
    let registers: Vec<&str> = owned.iter().map(|s| s.as_str()).collect();
    emit_call(out, native, &registers);
}

fn emit_call(out: &mut String, native: &str, registers: &[&str]) {
    out.push_str(&format!("    ncallr {HTML}, {native}, {HTML}"));
    for &register in registers {
        out.push_str(", ");
        out.push_str(register);
    }
    out.push('\n');
}

pub(super) fn load_arg(
    arg: &str,
    data: &HashMap<String, String>,
    scratch: &str,
    out: &mut String,
) -> String {
    if is_register(arg) {
        return arg.to_string();
    }

    if let Some(val) = data.get(arg) {
        let lit = flint_string_literal(val);
        out.push_str(&format!("    mov {scratch}, \"{lit}\"\n"));
        return scratch.to_string();
    }

    if arg.starts_with('"') {
        out.push_str(&format!("    mov {scratch}, {arg}\n"));
        return scratch.to_string();
    }

    // Unquoted identifier (e.g. `info` for alert kind) — wrap in quotes
    let lit = flint_string_literal(arg);
    out.push_str(&format!("    mov {scratch}, \"{lit}\"\n"));
    scratch.to_string()
}

fn is_register(s: &str) -> bool {
    s.strip_prefix('r')
        .and_then(|rest| rest.parse::<u8>().ok())
        .is_some_and(|n| n < 16)
}

pub(super) fn err_missing_arg(kw: &str, idx: usize, line_no: usize) -> PageCompileError {
    PageCompileError {
        line: line_no,
        message: format!("'{kw}' requires at least {} argument(s)", idx + 1),
    }
}
