//! 8086 Instruction

use crate::memory::Memory;
use crate::register::Register;

/// An 8086 instruction operand
#[derive(Debug, Copy, Clone)]
pub enum Operand {
    // A register operand
    Register(Register),

    // A memory operand
    Memory(Memory),

    // An immediate operand
    Immediate(i16),
}

/// REG field parsed from an instruction stream
#[derive(Debug, Copy, Clone)]
pub struct Reg(pub u8);

// W field parsed from an instruciton stream
#[derive(Debug, Copy, Clone)]
pub struct Wide(pub u8);

/// RM field parsed from an instruction stream
#[derive(Debug, Copy, Clone)]
pub struct Rm(pub u8);

/// MOD field parsed from an instruction stream
#[derive(Debug, Copy, Clone)]
pub struct Mod(pub u8);

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Operand::Register(reg) => write!(f, "{reg}"),
            Operand::Immediate(imm) => write!(f, "{imm:#x}"),
            Operand::Memory(memory) => {
                // If there is a direct memory address, write it and return
                if let Some(address) = memory.address {
                    // Start the memory bracket
                    write!(f, "{} [{address:#x}]", memory.size)?;
                    return Ok(());
                }

                // Start the memory bracket
                write!(f, "{} [", memory.size)?;

                // If the operand has a register(s), write them
                if let Some(reg1) = memory.registers[0] {
                    write!(f, "{reg1}")?;

                    if let Some(reg2) = memory.registers[1] {
                        write!(f, " + {reg2}")?;
                    }
                }

                // Write the displacement if it exists
                match memory.displacement {
                    Some(crate::memory::Displacement::Byte(offset)) => {
                        // Add pretty spacing around the sign of the offset
                        if offset.is_negative() {
                            write!(f, " - ")?;
                        } else {
                            write!(f, " + ")?;
                        }

                        // Regardless, print the absolute value of the offset
                        write!(f, "{:#x}", offset.abs())?;
                    }
                    Some(crate::memory::Displacement::Word(offset)) => {
                        // Add pretty spacing around the sign of the offset
                        if offset.is_negative() {
                            write!(f, " - ")?;
                        } else {
                            write!(f, " + ")?;
                        }

                        // Regardless, print the absolute value of the offset
                        write!(f, "{:#x}", offset.abs())?;
                    }
                    _ => {
                        // Nothing else to do here
                    }
                }

                write!(f, "]")
            }
        }
    }
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
            Instruction::Mov { dest, src } => {
                write!(f, "mov {dest}, {src}")
            }
        }
    }
}
