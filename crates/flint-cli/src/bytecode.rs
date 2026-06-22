use std::path::{Path, PathBuf};
use std::sync::Arc;

use flint::lang::{AppModule, Route};
use flint::vm::{Instr, Program, Reg, Value};

const MAGIC: &[u8; 8] = b"FLINTBC1";
const FORMAT_VERSION: u32 = 1;
// Public mixing constant used by the lightweight bytecode mask; not a secret.
const BYTECODE_MASK_MIX: u64 = 0x9e37_79b9_7f4a_7c15;

#[derive(Debug)]
pub struct BytecodeProject {
    pub name: String,
    pub version: String,
    pub modules: Vec<AppModule>,
}

pub fn file_name(project_name: &str) -> String {
    format!("{project_name}.flintbc")
}

pub fn is_bytecode_path(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "flintbc")
}

pub fn write_project(
    path: &Path,
    name: &str,
    version: &str,
    modules: &[AppModule],
) -> Result<(), String> {
    let encoded = encode_project(name, version, modules)?;
    std::fs::write(path, encoded)
        .map_err(|e| format!("cannot write bytecode '{}': {e}", path.display()))
}

pub fn read_project(path: &Path) -> Result<BytecodeProject, String> {
    let bytes = std::fs::read(path)
        .map_err(|e| format!("cannot read bytecode '{}': {e}", path.display()))?;
    decode_project(&bytes).map_err(|e| format!("{}: {e}", path.display()))
}

fn encode_project(name: &str, version: &str, modules: &[AppModule]) -> Result<Vec<u8>, String> {
    let mut payload = Vec::new();
    write_u32(&mut payload, FORMAT_VERSION);
    write_string(&mut payload, name)?;
    write_string(&mut payload, version)?;
    write_len(&mut payload, modules.len())?;
    for (index, module) in modules.iter().enumerate() {
        write_string(&mut payload, &format!("module:{index}"))?;
        write_program(&mut payload, &module.program)?;
        write_len(&mut payload, module.routes.len())?;
        for route in &module.routes {
            write_string(&mut payload, &route.method)?;
            write_string(&mut payload, &route.path)?;
            write_usize(&mut payload, route.handler_address)?;
        }
    }

    let checksum = checksum64(&payload);
    let seed = checksum
        ^ ((payload.len() as u64).rotate_left(17))
        ^ (modules.len() as u64).rotate_left(33)
        ^ BYTECODE_MASK_MIX;
    obfuscate(&mut payload, seed);

    let mut out = Vec::with_capacity(MAGIC.len() + 4 + 8 + 8 + 8 + payload.len());
    out.extend_from_slice(MAGIC);
    write_u32(&mut out, FORMAT_VERSION);
    write_u64(&mut out, seed);
    write_u64(&mut out, checksum);
    write_u64(&mut out, payload.len() as u64);
    out.extend_from_slice(&payload);
    Ok(out)
}

fn decode_project(bytes: &[u8]) -> Result<BytecodeProject, String> {
    let header_len = MAGIC.len() + 4 + 8 + 8 + 8;
    if bytes.len() < header_len || &bytes[..MAGIC.len()] != MAGIC {
        return Err("not a Flint bytecode file".to_string());
    }

    let mut header = Cursor::new(&bytes[MAGIC.len()..header_len]);
    let version = header.read_u32()?;
    if version != FORMAT_VERSION {
        return Err(format!("unsupported bytecode format version {version}"));
    }
    let seed = header.read_u64()?;
    let expected_checksum = header.read_u64()?;
    let payload_len = header.read_u64()? as usize;
    if bytes.len() != header_len + payload_len {
        return Err("bytecode file has an invalid payload length".to_string());
    }

    let mut payload = bytes[header_len..].to_vec();
    obfuscate(&mut payload, seed);
    let actual_checksum = checksum64(&payload);
    if actual_checksum != expected_checksum {
        return Err("bytecode checksum mismatch".to_string());
    }

    let mut cursor = Cursor::new(&payload);
    let payload_version = cursor.read_u32()?;
    if payload_version != FORMAT_VERSION {
        return Err(format!(
            "unsupported bytecode payload version {payload_version}"
        ));
    }

    let name = cursor.read_string()?;
    let version = cursor.read_string()?;
    let module_count = cursor.read_len()?;
    let mut modules = Vec::with_capacity(module_count);
    for _ in 0..module_count {
        let source_path = PathBuf::from(cursor.read_string()?);
        let program = Arc::new(read_program(&mut cursor)?);
        let route_count = cursor.read_len()?;
        let mut routes = Vec::with_capacity(route_count);
        for _ in 0..route_count {
            let method = cursor.read_string()?;
            let path = cursor.read_string()?;
            let handler_address = cursor.read_usize()?;
            routes.push(Route {
                method,
                path,
                handler: String::new(),
                handler_address,
            });
        }
        modules.push(AppModule {
            program,
            routes,
            source_path,
        });
    }

    if !cursor.is_finished() {
        return Err("bytecode payload has trailing bytes".to_string());
    }

    Ok(BytecodeProject {
        name,
        version,
        modules,
    })
}

fn write_program(out: &mut Vec<u8>, program: &Program) -> Result<(), String> {
    write_len(out, program.strings.len())?;
    for value in &program.strings {
        write_string(out, value)?;
    }

    write_len(out, program.initial_memory.len())?;
    for value in &program.initial_memory {
        write_value(out, value)?;
    }

    write_len(out, program.instructions.len())?;
    for instr in &program.instructions {
        write_instr(out, instr)?;
    }
    Ok(())
}

fn read_program(cursor: &mut Cursor<'_>) -> Result<Program, String> {
    let string_count = cursor.read_len()?;
    let mut strings = Vec::with_capacity(string_count);
    for _ in 0..string_count {
        strings.push(Arc::<str>::from(cursor.read_string()?));
    }

    let memory_count = cursor.read_len()?;
    let mut initial_memory = Vec::with_capacity(memory_count);
    for _ in 0..memory_count {
        initial_memory.push(cursor.read_value()?);
    }

    let instruction_count = cursor.read_len()?;
    let mut instructions = Vec::with_capacity(instruction_count);
    for _ in 0..instruction_count {
        instructions.push(cursor.read_instr()?);
    }

    Ok(Program {
        instructions,
        strings,
        initial_memory,
    })
}

fn write_instr(out: &mut Vec<u8>, instr: &Instr) -> Result<(), String> {
    match instr {
        Instr::LoadInt(dst, value) => write_op_reg_i64(out, 0x01, *dst, *value),
        Instr::LoadFloat(dst, value) => write_op_reg_u64(out, 0x02, *dst, value.to_bits()),
        Instr::LoadStr(dst, idx) => write_op_reg_u32(out, 0x03, *dst, *idx),
        Instr::Mov(dst, src) => write_op_reg_reg(out, 0x04, *dst, *src),
        Instr::Add(dst, a, b) => write_op_reg_reg_reg(out, 0x05, *dst, *a, *b),
        Instr::Sub(dst, a, b) => write_op_reg_reg_reg(out, 0x06, *dst, *a, *b),
        Instr::Mul(dst, a, b) => write_op_reg_reg_reg(out, 0x07, *dst, *a, *b),
        Instr::Div(dst, a, b) => write_op_reg_reg_reg(out, 0x08, *dst, *a, *b),
        Instr::Mod(dst, a, b) => write_op_reg_reg_reg(out, 0x09, *dst, *a, *b),
        Instr::AddImm(dst, a, imm) => write_op_reg_reg_i64(out, 0x0a, *dst, *a, *imm),
        Instr::SubImm(dst, a, imm) => write_op_reg_reg_i64(out, 0x0b, *dst, *a, *imm),
        Instr::MulImm(dst, a, imm) => write_op_reg_reg_i64(out, 0x0c, *dst, *a, *imm),
        Instr::DivImm(dst, a, imm) => write_op_reg_reg_i64(out, 0x0d, *dst, *a, *imm),
        Instr::ModImm(dst, a, imm) => write_op_reg_reg_i64(out, 0x0e, *dst, *a, *imm),
        Instr::AddF(dst, a, b) => write_op_reg_reg_reg(out, 0x0f, *dst, *a, *b),
        Instr::SubF(dst, a, b) => write_op_reg_reg_reg(out, 0x10, *dst, *a, *b),
        Instr::MulF(dst, a, b) => write_op_reg_reg_reg(out, 0x11, *dst, *a, *b),
        Instr::DivF(dst, a, b) => write_op_reg_reg_reg(out, 0x12, *dst, *a, *b),
        Instr::And(dst, a, b) => write_op_reg_reg_reg(out, 0x13, *dst, *a, *b),
        Instr::AndImm(dst, a, imm) => write_op_reg_reg_i64(out, 0x14, *dst, *a, *imm),
        Instr::Or(dst, a, b) => write_op_reg_reg_reg(out, 0x15, *dst, *a, *b),
        Instr::OrImm(dst, a, imm) => write_op_reg_reg_i64(out, 0x16, *dst, *a, *imm),
        Instr::Xor(dst, a, b) => write_op_reg_reg_reg(out, 0x17, *dst, *a, *b),
        Instr::XorImm(dst, a, imm) => write_op_reg_reg_i64(out, 0x18, *dst, *a, *imm),
        Instr::Not(dst, src) => write_op_reg_reg(out, 0x19, *dst, *src),
        Instr::Shl(dst, a, b) => write_op_reg_reg_reg(out, 0x1a, *dst, *a, *b),
        Instr::ShlImm(dst, a, imm) => write_op_reg_reg_i64(out, 0x1b, *dst, *a, *imm),
        Instr::Shr(dst, a, b) => write_op_reg_reg_reg(out, 0x1c, *dst, *a, *b),
        Instr::ShrImm(dst, a, imm) => write_op_reg_reg_i64(out, 0x1d, *dst, *a, *imm),
        Instr::Neg(dst, src) => write_op_reg_reg(out, 0x1e, *dst, *src),
        Instr::TypeOf(dst, src) => write_op_reg_reg(out, 0x1f, *dst, *src),
        Instr::Cmp(a, b) => write_op_reg_reg(out, 0x20, *a, *b),
        Instr::CmpImm(a, imm) => write_op_reg_i64(out, 0x21, *a, *imm),
        Instr::Jmp(target) => write_op_usize(out, 0x22, *target),
        Instr::Je(target) => write_op_usize(out, 0x23, *target),
        Instr::Jne(target) => write_op_usize(out, 0x24, *target),
        Instr::Jl(target) => write_op_usize(out, 0x25, *target),
        Instr::Jg(target) => write_op_usize(out, 0x26, *target),
        Instr::Jle(target) => write_op_usize(out, 0x27, *target),
        Instr::Jge(target) => write_op_usize(out, 0x28, *target),
        Instr::Push(src) => write_op_reg(out, 0x29, *src),
        Instr::Pop(dst) => write_op_reg(out, 0x2a, *dst),
        Instr::Call(target) => write_op_usize(out, 0x2b, *target),
        Instr::Ret => write_op(out, 0x2c),
        Instr::Load(dst, addr) => write_op_reg_reg(out, 0x2d, *dst, *addr),
        Instr::Store(addr, src) => write_op_reg_reg(out, 0x2e, *addr, *src),
        Instr::NCall {
            name_idx,
            args,
            dst,
        } => {
            write_op(out, 0x2f)?;
            write_u32(out, *name_idx);
            match dst {
                Some(reg) => out.push(*reg),
                None => out.push(0xff),
            }
            write_len(out, args.len())?;
            out.extend(args.iter().copied());
            Ok(())
        }
        Instr::Hlt => write_op(out, 0x30),
    }
}

fn write_value(out: &mut Vec<u8>, value: &Value) -> Result<(), String> {
    match value {
        Value::Int(value) => {
            out.push(0x01);
            write_i64(out, *value);
        }
        Value::Float(value) => {
            out.push(0x02);
            write_u64(out, value.to_bits());
        }
        Value::Str(value) => {
            out.push(0x03);
            write_string(out, value)?;
        }
        Value::Json(value) => {
            out.push(0x04);
            write_string(out, &value.to_string())?;
        }
    }
    Ok(())
}

fn write_op(out: &mut Vec<u8>, op: u8) -> Result<(), String> {
    out.push(op);
    Ok(())
}

fn write_op_reg(out: &mut Vec<u8>, op: u8, reg: Reg) -> Result<(), String> {
    write_op(out, op)?;
    out.push(reg);
    Ok(())
}

fn write_op_reg_reg(out: &mut Vec<u8>, op: u8, a: Reg, b: Reg) -> Result<(), String> {
    write_op_reg(out, op, a)?;
    out.push(b);
    Ok(())
}

fn write_op_reg_reg_reg(out: &mut Vec<u8>, op: u8, a: Reg, b: Reg, c: Reg) -> Result<(), String> {
    write_op_reg_reg(out, op, a, b)?;
    out.push(c);
    Ok(())
}

fn write_op_reg_i64(out: &mut Vec<u8>, op: u8, reg: Reg, value: i64) -> Result<(), String> {
    write_op_reg(out, op, reg)?;
    write_i64(out, value);
    Ok(())
}

fn write_op_reg_u32(out: &mut Vec<u8>, op: u8, reg: Reg, value: u32) -> Result<(), String> {
    write_op_reg(out, op, reg)?;
    write_u32(out, value);
    Ok(())
}

fn write_op_reg_u64(out: &mut Vec<u8>, op: u8, reg: Reg, value: u64) -> Result<(), String> {
    write_op_reg(out, op, reg)?;
    write_u64(out, value);
    Ok(())
}

fn write_op_reg_reg_i64(
    out: &mut Vec<u8>,
    op: u8,
    a: Reg,
    b: Reg,
    value: i64,
) -> Result<(), String> {
    write_op_reg_reg(out, op, a, b)?;
    write_i64(out, value);
    Ok(())
}

fn write_op_usize(out: &mut Vec<u8>, op: u8, value: usize) -> Result<(), String> {
    write_op(out, op)?;
    write_usize(out, value)
}

fn write_len(out: &mut Vec<u8>, value: usize) -> Result<(), String> {
    let value = u32::try_from(value).map_err(|_| "bytecode section is too large".to_string())?;
    write_u32(out, value);
    Ok(())
}

fn write_usize(out: &mut Vec<u8>, value: usize) -> Result<(), String> {
    let value = u32::try_from(value).map_err(|_| "bytecode address is too large".to_string())?;
    write_u32(out, value);
    Ok(())
}

fn write_string(out: &mut Vec<u8>, value: &str) -> Result<(), String> {
    write_len(out, value.len())?;
    out.extend_from_slice(value.as_bytes());
    Ok(())
}

fn write_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn write_u64(out: &mut Vec<u8>, value: u64) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn write_i64(out: &mut Vec<u8>, value: i64) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn obfuscate(bytes: &mut [u8], seed: u64) {
    let mut state = seed ^ BYTECODE_MASK_MIX;
    for (i, byte) in bytes.iter_mut().enumerate() {
        state = splitmix64(state.wrapping_add((i as u64) ^ BYTECODE_MASK_MIX));
        let mask = (state.rotate_left((i % 63) as u32) & 0xff) as u8;
        *byte ^= mask;
    }
}

fn checksum64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

fn splitmix64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9e37_79b9_7f4a_7c15);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

struct Cursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }

    fn is_finished(&self) -> bool {
        self.pos == self.bytes.len()
    }

    fn take(&mut self, len: usize) -> Result<&'a [u8], String> {
        let end = self
            .pos
            .checked_add(len)
            .ok_or_else(|| "bytecode cursor overflow".to_string())?;
        if end > self.bytes.len() {
            return Err("bytecode ended unexpectedly".to_string());
        }
        let slice = &self.bytes[self.pos..end];
        self.pos = end;
        Ok(slice)
    }

    fn read_u8(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }

    fn read_u32(&mut self) -> Result<u32, String> {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(self.take(4)?);
        Ok(u32::from_le_bytes(bytes))
    }

    fn read_u64(&mut self) -> Result<u64, String> {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(self.take(8)?);
        Ok(u64::from_le_bytes(bytes))
    }

    fn read_i64(&mut self) -> Result<i64, String> {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(self.take(8)?);
        Ok(i64::from_le_bytes(bytes))
    }

    fn read_len(&mut self) -> Result<usize, String> {
        Ok(self.read_u32()? as usize)
    }

    fn read_usize(&mut self) -> Result<usize, String> {
        Ok(self.read_u32()? as usize)
    }

    fn read_string(&mut self) -> Result<String, String> {
        let len = self.read_len()?;
        let bytes = self.take(len)?;
        String::from_utf8(bytes.to_vec()).map_err(|_| "bytecode contains invalid UTF-8".to_string())
    }

    fn read_reg(&mut self) -> Result<Reg, String> {
        let reg = self.read_u8()?;
        if reg >= flint::vm::NUM_REGISTERS as u8 {
            return Err(format!("bytecode register r{reg} is out of range"));
        }
        Ok(reg)
    }

    fn read_value(&mut self) -> Result<Value, String> {
        match self.read_u8()? {
            0x01 => Ok(Value::Int(self.read_i64()?)),
            0x02 => Ok(Value::Float(f64::from_bits(self.read_u64()?))),
            0x03 => Ok(Value::Str(Arc::<str>::from(self.read_string()?))),
            0x04 => {
                let json = self.read_string()?;
                let value = serde_json::from_str(&json)
                    .map_err(|e| format!("bytecode contains invalid JSON value: {e}"))?;
                Ok(Value::Json(Arc::new(value)))
            }
            tag => Err(format!("unknown bytecode value tag 0x{tag:02x}")),
        }
    }

    fn read_instr(&mut self) -> Result<Instr, String> {
        match self.read_u8()? {
            0x01 => Ok(Instr::LoadInt(self.read_reg()?, self.read_i64()?)),
            0x02 => Ok(Instr::LoadFloat(
                self.read_reg()?,
                f64::from_bits(self.read_u64()?),
            )),
            0x03 => Ok(Instr::LoadStr(self.read_reg()?, self.read_u32()?)),
            0x04 => Ok(Instr::Mov(self.read_reg()?, self.read_reg()?)),
            0x05 => self.read_rrr(Instr::Add),
            0x06 => self.read_rrr(Instr::Sub),
            0x07 => self.read_rrr(Instr::Mul),
            0x08 => self.read_rrr(Instr::Div),
            0x09 => self.read_rrr(Instr::Mod),
            0x0a => self.read_rri(Instr::AddImm),
            0x0b => self.read_rri(Instr::SubImm),
            0x0c => self.read_rri(Instr::MulImm),
            0x0d => self.read_rri(Instr::DivImm),
            0x0e => self.read_rri(Instr::ModImm),
            0x0f => self.read_rrr(Instr::AddF),
            0x10 => self.read_rrr(Instr::SubF),
            0x11 => self.read_rrr(Instr::MulF),
            0x12 => self.read_rrr(Instr::DivF),
            0x13 => self.read_rrr(Instr::And),
            0x14 => self.read_rri(Instr::AndImm),
            0x15 => self.read_rrr(Instr::Or),
            0x16 => self.read_rri(Instr::OrImm),
            0x17 => self.read_rrr(Instr::Xor),
            0x18 => self.read_rri(Instr::XorImm),
            0x19 => Ok(Instr::Not(self.read_reg()?, self.read_reg()?)),
            0x1a => self.read_rrr(Instr::Shl),
            0x1b => self.read_rri(Instr::ShlImm),
            0x1c => self.read_rrr(Instr::Shr),
            0x1d => self.read_rri(Instr::ShrImm),
            0x1e => Ok(Instr::Neg(self.read_reg()?, self.read_reg()?)),
            0x1f => Ok(Instr::TypeOf(self.read_reg()?, self.read_reg()?)),
            0x20 => Ok(Instr::Cmp(self.read_reg()?, self.read_reg()?)),
            0x21 => Ok(Instr::CmpImm(self.read_reg()?, self.read_i64()?)),
            0x22 => Ok(Instr::Jmp(self.read_usize()?)),
            0x23 => Ok(Instr::Je(self.read_usize()?)),
            0x24 => Ok(Instr::Jne(self.read_usize()?)),
            0x25 => Ok(Instr::Jl(self.read_usize()?)),
            0x26 => Ok(Instr::Jg(self.read_usize()?)),
            0x27 => Ok(Instr::Jle(self.read_usize()?)),
            0x28 => Ok(Instr::Jge(self.read_usize()?)),
            0x29 => Ok(Instr::Push(self.read_reg()?)),
            0x2a => Ok(Instr::Pop(self.read_reg()?)),
            0x2b => Ok(Instr::Call(self.read_usize()?)),
            0x2c => Ok(Instr::Ret),
            0x2d => Ok(Instr::Load(self.read_reg()?, self.read_reg()?)),
            0x2e => Ok(Instr::Store(self.read_reg()?, self.read_reg()?)),
            0x2f => {
                let name_idx = self.read_u32()?;
                let dst = match self.read_u8()? {
                    0xff => None,
                    reg if reg < flint::vm::NUM_REGISTERS as u8 => Some(reg),
                    reg => return Err(format!("bytecode register r{reg} is out of range")),
                };
                let arg_count = self.read_len()?;
                let mut args = Vec::with_capacity(arg_count);
                for _ in 0..arg_count {
                    args.push(self.read_reg()?);
                }
                Ok(Instr::NCall {
                    name_idx,
                    args,
                    dst,
                })
            }
            0x30 => Ok(Instr::Hlt),
            op => Err(format!("unknown bytecode opcode 0x{op:02x}")),
        }
    }

    fn read_rrr(&mut self, ctor: fn(Reg, Reg, Reg) -> Instr) -> Result<Instr, String> {
        Ok(ctor(self.read_reg()?, self.read_reg()?, self.read_reg()?))
    }

    fn read_rri(&mut self, ctor: fn(Reg, Reg, i64) -> Instr) -> Result<Instr, String> {
        Ok(ctor(self.read_reg()?, self.read_reg()?, self.read_i64()?))
    }
}

#[cfg(test)]
mod tests {
    use super::{decode_project, encode_project};
    use flint::lang::{AppModule, Route};
    use std::path::PathBuf;
    use std::sync::Arc;

    fn module(source: &str) -> AppModule {
        let app = flint::lang::compile_app_source(source).unwrap();
        AppModule {
            program: Arc::new(app.program),
            routes: app.routes,
            source_path: PathBuf::from("routes/test.fl"),
        }
    }

    #[test]
    fn bytecode_round_trips_a_compiled_module_without_source() {
        let modules = vec![module(
            r#"section .route
    GET "/hello" -> hello

section .data
message:
    data "Hello"

section .text
hello:
    mov r0, message
    load r1, [r0]
    ncall http.text, r1
    ret
"#,
        )];

        let encoded = encode_project("demo", "1.0.0", &modules).unwrap();
        assert!(!encoded
            .windows(b"section .text".len())
            .any(|w| w == b"section .text"));
        assert!(!encoded.windows(b"Hello".len()).any(|w| w == b"Hello"));

        let decoded = decode_project(&encoded).unwrap();
        assert_eq!(decoded.name, "demo");
        assert_eq!(decoded.version, "1.0.0");
        assert_eq!(decoded.modules.len(), 1);
        assert_eq!(decoded.modules[0].routes.len(), 1);
        assert_eq!(decoded.modules[0].routes[0].method, "GET");
        assert_eq!(decoded.modules[0].routes[0].path, "/hello");
        assert!(decoded.modules[0].program.instructions.len() >= 4);
    }

    #[test]
    fn rejects_corrupted_payloads() {
        let modules = vec![module(
            "section .route\n    GET \"/\" -> home\n\nsection .text\nhome:\n    ret\n",
        )];
        let mut encoded = encode_project("demo", "1.0.0", &modules).unwrap();
        let last = encoded.len() - 1;
        encoded[last] ^= 0x55;

        let err = decode_project(&encoded).unwrap_err();
        assert!(err.contains("checksum"), "{err}");
    }

    #[test]
    fn routes_do_not_need_handler_names_at_runtime() {
        let modules = vec![module(
            "section .route\n    GET \"/\" -> home\n\nsection .text\nhome:\n    ret\n",
        )];
        let encoded = encode_project("demo", "1.0.0", &modules).unwrap();
        let decoded = decode_project(&encoded).unwrap();

        assert_eq!(
            decoded.modules[0].routes[0],
            Route {
                method: "GET".to_string(),
                path: "/".to_string(),
                handler: String::new(),
                handler_address: 0,
            }
        );
    }
}
