use std::fmt;

/// An error raised while executing a compiled program.
#[derive(Debug, Clone, PartialEq)]
pub struct VmError {
    /// Index of the instruction that triggered the error.
    pub pc: usize,
    pub message: String,
}

impl VmError {
    pub fn new(pc: usize, message: impl Into<String>) -> Self {
        Self {
            pc,
            message: message.into(),
        }
    }
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "runtime error at instruction {}: {}",
            self.pc, self.message
        )
    }
}

impl std::error::Error for VmError {}
