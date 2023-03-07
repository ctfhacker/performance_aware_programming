//! A memory operand

use anyhow::Result;
use thiserror::Error;

use crate::instruction::{Mod, Rm, Wide};
use crate::register::Register;

/// Possible errors while handling memory operands
#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("0b11 mod value is not used with memory encoding")]
    AttemptedThreeModInMemory,

    #[error("Invalid rm value found while creating memory operand: {0:#x}")]
    InvalidRMValue(u8),
}

/// Size of a memory read
#[derive(Debug, Copy, Clone)]
pub enum MemorySize {
    Byte,
    Word,
}

impl std::fmt::Display for MemorySize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            MemorySize::Byte => write!(f, "byte"),
            MemorySize::Word => write!(f, "word"),
        }
    }
}

/// A memory operand
#[derive(Debug, Copy, Clone)]
pub struct Memory {
    /// Registers involved with this memory operand
    pub registers: [Option<Register>; 2],

    /// Displacement value for this memory operand
    pub displacement: Option<i16>,

    /// Size of memory read
    pub size: MemorySize,

    /// Direct address for this memory operand
    pub address: Option<u16>,
}

impl Memory {
    /// Create a direct address memory operand
    pub fn direct_address(addr: u16) -> Memory {
        Memory {
            registers: [None; 2],
            displacement: None,
            size: MemorySize::Word,
            address: Some(addr),
        }
    }

    pub fn from_mod_rm(mod_: Mod, rm: Rm, wide: Wide) -> Result<Memory> {
        let mod_ = mod_.0;
        let rm = rm.0;

        if mod_ == 0b11 {
            return Err(MemoryError::AttemptedThreeModInMemory.into());
        }

        let mut registers = [None; 2];
        match rm {
            0b000 => {
                registers[0] = Some(Register::Bx);
                registers[1] = Some(Register::Si);
            }
            0b001 => {
                registers[0] = Some(Register::Bx);
                registers[1] = Some(Register::Di);
            }
            0b010 => {
                registers[0] = Some(Register::Bp);
                registers[1] = Some(Register::Si);
            }
            0b011 => {
                registers[0] = Some(Register::Bp);
                registers[1] = Some(Register::Di);
            }
            0b100 => {
                registers[0] = Some(Register::Si);
            }
            0b101 => {
                registers[0] = Some(Register::Di);
            }
            0b110 => {
                if mod_ != 0b00 {
                    registers[0] = Some(Register::Bp);
                } else {
                    // No registers set. RM b110 is Direct Address
                }
            }
            0b111 => {
                registers[0] = Some(Register::Bx);
            }
            _ => return Err(MemoryError::InvalidRMValue(rm).into()),
        }

        let size = match wide.0 {
            0 => MemorySize::Byte,
            1 => MemorySize::Word,
            _ => unsafe { std::hint::unreachable_unchecked() },
        };

        Ok(Memory {
            registers,
            displacement: None,
            size,
            address: None,
        })
    }

    /// Set the displacement for this memory operand
    pub fn with_displacement(mut self, displacement: i16) -> Self {
        self.displacement = Some(displacement);
        self
    }
}
