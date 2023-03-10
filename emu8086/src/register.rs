//! An 8086 register
use crate::instruction::{Reg, Wide};

use thiserror::Error;

/// Register 8086 bank
#[derive(Debug, Copy, Clone)]
pub enum Register {
    Ax,
    Al,
    Ah,
    Bx,
    Bl,
    Bh,
    Cx,
    Cl,
    Ch,
    Dx,
    Dl,
    Dh,
    Si,
    Di,
    Sp,
    Bp,
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Register::Ax => write!(f, "ax"),
            Register::Al => write!(f, "al"),
            Register::Ah => write!(f, "ah"),
            Register::Bx => write!(f, "bx"),
            Register::Bl => write!(f, "bl"),
            Register::Bh => write!(f, "bh"),
            Register::Cx => write!(f, "cx"),
            Register::Cl => write!(f, "cl"),
            Register::Ch => write!(f, "ch"),
            Register::Dx => write!(f, "dx"),
            Register::Dl => write!(f, "dl"),
            Register::Dh => write!(f, "dh"),
            Register::Si => write!(f, "si"),
            Register::Di => write!(f, "di"),
            Register::Sp => write!(f, "sp"),
            Register::Bp => write!(f, "bp"),
        }
    }
}

impl Register {
    // Get a register from a decoded `reg` or `rm` value and `w`
    pub const fn from_reg_w(reg: Reg, w: Wide) -> Register {
        match (reg.0, w.0) {
            (0b000, 0b0) => Register::Al,
            (0b000, 0b1) => Register::Ax,
            (0b001, 0b0) => Register::Cl,
            (0b001, 0b1) => Register::Cx,
            (0b010, 0b0) => Register::Dl,
            (0b010, 0b1) => Register::Dx,
            (0b011, 0b0) => Register::Bl,
            (0b011, 0b1) => Register::Bx,
            (0b100, 0b0) => Register::Ah,
            (0b100, 0b1) => Register::Sp,
            (0b101, 0b0) => Register::Ch,
            (0b101, 0b1) => Register::Bp,
            (0b110, 0b0) => Register::Dh,
            (0b110, 0b1) => Register::Si,
            (0b111, 0b0) => Register::Bh,
            (0b111, 0b1) => Register::Di,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}

/// An 8086 segment register
#[derive(Debug, Copy, Clone)]
pub enum SegmentRegister {
    Es,
    Cs,
    Ss,
    Ds,
}

/// Error while parsing a segment register
#[derive(Error, Debug)]
pub enum SegmentRegisterError {
    #[error("Attempted to parse an unknown segment register: {0:#x}")]
    UnknownSegmentRegister(u8),
}

impl TryFrom<u8> for SegmentRegister {
    type Error = SegmentRegisterError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00 => Ok(SegmentRegister::Es),
            0b01 => Ok(SegmentRegister::Cs),
            0b10 => Ok(SegmentRegister::Ss),
            0b11 => Ok(SegmentRegister::Ds),
            _ => Err(SegmentRegisterError::UnknownSegmentRegister(value)),
        }
    }
}

impl std::fmt::Display for SegmentRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            SegmentRegister::Es => write!(f, "es"),
            SegmentRegister::Cs => write!(f, "cs"),
            SegmentRegister::Ss => write!(f, "ss"),
            SegmentRegister::Ds => write!(f, "ds"),
        }
    }
}
