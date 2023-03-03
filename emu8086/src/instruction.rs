//! 8086 Instruction

use crate::register::Register;

/// An 8086 instruction operand
#[derive(Debug)]
pub enum Operand {
    // A register operand
    Register(Register),
}

/// An 8086 decoded instruction
#[derive(Debug)]
pub enum Instruction {
    /// A mov instruction
    ///
    /// Example:
    /// mov cx, bx
    Mov { dest: Operand, src: Operand },
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Instruction::Mov {
                dest: Operand::Register(dest),
                src: Operand::Register(src),
            } => {
                write!(f, "mov {dest}, {src}")
            }
        }
    }
}
