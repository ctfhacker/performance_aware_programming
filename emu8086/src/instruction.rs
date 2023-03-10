//! 8086 Instruction

use crate::memory::Memory;
use crate::register::{Register, SegmentRegister};

/// An 8086 instruction operand
#[derive(Debug, Copy, Clone)]
pub enum Operand {
    // A register operand
    Register(Register),

    // A memory operand
    Memory(Memory),

    // An immediate operand
    Immediate(i16),

    // A segment register
    SegmentRegister(SegmentRegister),
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
            Operand::SegmentRegister(segreg) => write!(f, "{segreg}"),
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
                if let Some(displacement) = memory.displacement {
                    // Add pretty spacing around the sign of the offset
                    if displacement.is_negative() {
                        write!(f, " - ")?;
                    } else {
                        write!(f, " + ")?;
                    }

                    // Regardless, print the absolute value of the offset
                    write!(f, "{:#x}", displacement.abs())?;
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

    /// A push instruction
    ///
    /// Example:
    /// push ax
    /// push word [bp]
    /// push word [0x1234]
    /// push word [bx + si]
    Push { src: Operand },

    /// A pop instruction
    ///
    /// Example:
    /// pop ax
    /// pop word [bp]
    /// pop word [0x1234]
    /// pop word [bx + si]
    Pop { src: Operand },

    /// An exchange (xchg) instruction
    Xchg { left: Operand, right: Operand },

    /// A no-operation
    Nop,

    /// Read from port (IN operation)
    In { dest: Register, src: Operand },

    /// Write to port (IN operation)
    Out { dest: Operand, src: Register },

    /// Table Look-up Translation
    ///
    /// Locates a byte entry in a table in memory, using the contents of the AL register as a table
    /// index, then copies the contents of the table entry back into the AL register. The index in
    /// the AL register is treated as an unsigned integer. The XLAT and XLATB instructions get the
    /// base address of the table in memory from either the DS:EBX or the DS:BX registers (depending
    /// on the address-size attribute of the instruction, 32 or 16, respectively). (The DS segment
    /// may be overridden with a segment override prefix.)
    ///
    /// IF AddressSize = 16
    /// THEN
    ///     AL := (DS:BX + ZeroExtend(AL));
    /// ELSE IF (AddressSize = 32)
    ///     AL := (DS:EBX + ZeroExtend(AL)); FI;
    /// ELSE (AddressSize = 64)
    ///     AL := (RBX + ZeroExtend(AL));
    /// FI;
    Xlat,

    /// Load effective address
    Lea { dest: Operand, src: Operand },

    /// Load pointer to DS
    Lds { src: Operand },

    /// Load pointer to ES
    Les { src: Operand },

    /// Load AH with flags
    Lahf,

    /// Store AH into flags
    Sahf,

    /// Push flags
    Pushf,

    /// Pop flags
    Popf,

    /// An add instruction
    Add { dest: Operand, src: Operand },

    /// An add with carry instruction
    Adc { dest: Operand, src: Operand },

    /// An and instruction
    And { dest: Operand, src: Operand },

    /// A test instruction
    Test { dest: Operand, src: Operand },

    /// A test instruction
    Or { dest: Operand, src: Operand },

    /// A xor instruction
    Xor { dest: Operand, src: Operand },

    /// An increment instruction
    Inc { dest: Operand },

    /// An decrement instruction
    Dec { dest: Operand },

    /// ASCII adjust for add
    Aaa,

    /// Decimal adjust for add
    Daa,

    /// ASCII adjust after subtraction
    Aas,

    /// Decimal adjust after subtraction
    Das,

    /// ASCII adjust after multiplication
    Aam,

    /// Decimal adjust after division
    Aad,

    /// A subtraction operation
    Sub { dest: Operand, src: Operand },

    /// A subtraction operation with borrow
    Sbb { dest: Operand, src: Operand },

    /// A multiplication operation
    Mul { dest: Operand, src: Operand },

    /// A signed multiplication operation
    Imul { dest: Operand, src: Operand },

    /// A divide operation
    Div { dest: Operand, src: Operand },

    /// A signed divide operation
    Idiv { dest: Operand, src: Operand },

    /// A change sign operation
    Neg { src: Operand },

    /// A comparison operation
    Cmp { dest: Operand, src: Operand },

    /// Convert byte to word
    Cbw,

    /// Convert word to double word
    Cwd,

    /// An inversion opeation
    Not { src: Operand },

    /// Shift logical left
    Shl { src: Operand, count: Operand },

    /// Shift arithmetic left
    Sal { src: Operand, count: Operand },

    /// Shift arithmetic right
    Sar { src: Operand, count: Operand },

    /// Shift logical right
    Shr { src: Operand, count: Operand },

    /// Rotate left
    Rol { src: Operand, count: Operand },

    /// Rotate right
    Ror { src: Operand, count: Operand },

    /// Rotate through carry flag left
    Rcl { src: Operand, count: Operand },

    /// Rotate through carry flag right
    Rcr { src: Operand, count: Operand },
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Instruction::Mov { dest, src } => {
                write!(f, "mov {dest}, {src}")
            }
            Instruction::Push { src } => {
                write!(f, "push {src}")
            }
            Instruction::Pop { src } => {
                write!(f, "pop {src}")
            }
            Instruction::Xchg { left, right } => {
                write!(f, "xchg {left}, {right}")
            }
            Instruction::Nop => {
                write!(f, "nop")
            }
            Instruction::In { dest, src } => {
                write!(f, "in {dest}, {src}")
            }
            Instruction::Out { dest, src } => {
                write!(f, "out {dest}, {src}")
            }
            Instruction::Xlat => {
                write!(f, "xlat")
            }
            Instruction::Lea { dest, src } => {
                write!(f, "lea {dest}, {src}")
            }
            Instruction::Lds { src } => {
                write!(f, "lds {src}")
            }
            Instruction::Les { src } => {
                write!(f, "les {src}")
            }
            Instruction::Lahf => {
                write!(f, "lahf")
            }
            Instruction::Sahf => {
                write!(f, "sahf")
            }
            Instruction::Pushf => {
                write!(f, "pushf")
            }
            Instruction::Popf => {
                write!(f, "popf")
            }
            Instruction::Add { dest, src } => {
                write!(f, "add {dest}, {src}")
            }
            Instruction::And { dest, src } => {
                write!(f, "and {dest}, {src}")
            }
            Instruction::Test { dest, src } => {
                write!(f, "test {dest}, {src}")
            }
            Instruction::Or { dest, src } => {
                write!(f, "or {dest}, {src}")
            }
            Instruction::Xor { dest, src } => {
                write!(f, "xor {dest}, {src}")
            }
            Instruction::Adc { dest, src } => {
                write!(f, "adc {dest}, {src}")
            }
            Instruction::Inc { dest } => {
                write!(f, "inc {dest}")
            }
            Instruction::Dec { dest } => {
                write!(f, "dec {dest}")
            }
            Instruction::Aaa => {
                write!(f, "aaa")
            }
            Instruction::Daa => {
                write!(f, "daa")
            }
            Instruction::Aas => {
                write!(f, "aas")
            }
            Instruction::Das => {
                write!(f, "das")
            }
            Instruction::Aam => {
                write!(f, "aam")
            }
            Instruction::Aad => {
                write!(f, "aad")
            }
            Instruction::Sub { dest, src } => {
                write!(f, "sub {dest}, {src}")
            }
            Instruction::Sbb { dest, src } => {
                write!(f, "sbb {dest}, {src}")
            }
            Instruction::Mul { dest, src } => {
                write!(f, "mul {dest}, {src}")
            }
            Instruction::Imul { dest, src } => {
                write!(f, "imul {dest}, {src}")
            }
            Instruction::Div { dest, src } => {
                write!(f, "div {dest}, {src}")
            }
            Instruction::Idiv { dest, src } => {
                write!(f, "idiv {dest}, {src}")
            }
            Instruction::Cmp { dest, src } => {
                write!(f, "cmp  {dest}, {src}")
            }
            Instruction::Neg { src } => {
                write!(f, "neg {src}")
            }
            Instruction::Cbw => {
                write!(f, "cbw")
            }
            Instruction::Cwd => {
                write!(f, "cwd")
            }
            Instruction::Not { src } => {
                write!(f, "not {src}")
            }
            Instruction::Shl { src, count } => {
                write!(f, "shl {src}, {count}")
            }
            Instruction::Sar { src, count } => {
                write!(f, "sar {src}, {count}")
            }
            Instruction::Shr { src, count } => {
                write!(f, "shr {src}, {count}")
            }
            Instruction::Sal { src, count } => {
                write!(f, "sal {src}, {count}")
            }
            Instruction::Rol { src, count } => {
                write!(f, "rol {src}, {count}")
            }
            Instruction::Ror { src, count } => {
                write!(f, "ror {src}, {count}")
            }
            Instruction::Rcl { src, count } => {
                write!(f, "rcl {src}, {count}")
            }
            Instruction::Rcr { src, count } => {
                write!(f, "rcr {src}, {count}")
            }
        }
    }
}
