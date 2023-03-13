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
                    if let Some(size) = memory.size {
                        write!(f, "{} ", size)?;
                    }

                    if let Some(segment) = memory.segment {
                        write!(f, "{segment}:")?;
                    }

                    write!(f, "[{address:#x}]")?;
                    return Ok(());
                }

                // Start the memory bracket
                if let Some(size) = memory.size {
                    write!(f, "{} ", size)?;
                }

                // If a segment exists, insert it here
                if let Some(segment) = memory.segment {
                    write!(f, "{segment}:")?;
                }

                // Open the memory bracket
                write!(f, "[")?;

                // If the operand has a register(s), write them
                if let Some(reg1) = memory.registers[0] {
                    write!(f, "{reg1}")?;

                    if let Some(reg2) = memory.registers[1] {
                        write!(f, " + {reg2}")?;
                    }
                }

                // Write the displacement if it exists
                if let Some(displacement) = memory.displacement {
                    if displacement != 0 {
                        // Add pretty spacing around the sign of the offset
                        if displacement.is_negative() {
                            write!(f, " - ")?;
                        } else {
                            write!(f, " + ")?;
                        }

                        // Regardless, print the absolute value of the offset
                        write!(f, "{:#x}", displacement.abs())?;
                    }
                }

                write!(f, "]")
            }
        }
    }
}

/// Repeat prefix for string manipulation instructions
#[derive(Debug, Copy, Clone)]
pub enum Repeat {
    /// Repeat/loop while zero flag is clear
    WhileClearZeroFlag,

    /// Repeat/loop while zero flag is set
    WhileSetZeroFlag,
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
    Lds { dest: Operand, src: Operand },

    /// Load pointer to ES
    Les { dest: Operand, src: Operand },

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
    Inc { src: Operand },

    /// An decrement instruction
    Dec { src: Operand },

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
    Mul { src: Operand },

    /// A signed multiplication operation
    Imul { src: Operand },

    /// A divide operation
    Div { src: Operand },

    /// A signed divide operation
    Idiv { src: Operand },

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

    /// Move byte
    MoveByte { repeat: Option<Repeat> },

    /// Move word
    MoveWord { repeat: Option<Repeat> },

    /// Compare byte
    CmpByte { repeat: Option<Repeat> },

    /// Compare word
    CmpWord { repeat: Option<Repeat> },

    /// Scan byte
    ScanByte { repeat: Option<Repeat> },

    /// Scan word
    ScanWord { repeat: Option<Repeat> },

    /// Load byte to AL
    LoadByte { repeat: Option<Repeat> },

    /// Load word to AX
    LoadWord { repeat: Option<Repeat> },

    /// Store byte to AL
    StoreByte { repeat: Option<Repeat> },

    /// Store word to AX
    StoreWord { repeat: Option<Repeat> },

    /// Call
    Call { dest: Operand },

    /// Jump
    Jump { dest: Operand },

    /// Return and adjust the stack based on offset
    ReturnWithOffset { offset: i16 },

    /// Return
    Return,

    /// Jump on equal/zero
    JumpEqual { offset: i8, label: Option<String> },

    /// Jump on less/not greater or equal
    JumpLessThan { offset: i8, label: Option<String> },

    /// Jump on lessor or equal/not greater
    JumpLessThanEqual { offset: i8, label: Option<String> },

    /// Jump on below/not above or equal
    JumpBelow { offset: i8, label: Option<String> },

    /// Jump on below or equal/not above
    JumpBelowEqual { offset: i8, label: Option<String> },

    /// Jump on parity/parity even
    JumpParityEven { offset: i8, label: Option<String> },

    /// Jump on overflow
    JumpOverflow { offset: i8, label: Option<String> },

    /// Jump on sign
    JumpSign { offset: i8, label: Option<String> },

    /// Jump on not parity/parity odd
    JumpParityOdd { offset: i8, label: Option<String> },

    /// Jump on not equal/not zero
    JumpNotEqual { offset: i8, label: Option<String> },

    /// Jump on not less/greater or equal
    JumpNotLessThan { offset: i8, label: Option<String> },

    /// Jump on not less or equal/greater
    JumpNotLessThanEqual { offset: i8, label: Option<String> },

    /// Jump on not below/above or equal
    JumpNotBelow { offset: i8, label: Option<String> },

    /// Jump on not below or equal/above
    JumpNotBelowEqual { offset: i8, label: Option<String> },

    /// Jump on not overflow
    JumpNotOverflow { offset: i8, label: Option<String> },

    /// Jump on not sign
    JumpNotSign { offset: i8, label: Option<String> },

    /// Loop CX times
    Loop { offset: i8, label: Option<String> },

    /// Loop while zero
    LoopWhileZero { offset: i8, label: Option<String> },

    /// Loop while not zero
    LoopWhileNotZero { offset: i8, label: Option<String> },

    /// Jump on CX zero
    JumpCxZero { offset: i8, label: Option<String> },

    /// Interrupt on vector
    Interrupt { vector: u8 },

    /// Interrupt on overflow
    InterruptOnOverflow,

    /// Return from interrupt
    InterruptReturn,

    /// Clear the carry bit
    ClearCarry,

    /// Complement carry
    ComplementCarry,

    /// Set carry
    SetCarry,

    /// Clear direction
    ClearDirection,

    /// Set direction bit
    SetDirection,

    /// Clear interrupt bit
    ClearInterrupt,

    /// Set interrupt bit
    SetInterrupt,

    /// Halt
    Halt,

    /// Wait
    Wait,

    /// Lock prefix
    Lock,

    /// A jump label
    Label { name: String },
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
            Instruction::Lds { dest, src } => {
                write!(f, "lds {dest}, {src}")
            }
            Instruction::Les { dest, src } => {
                write!(f, "les {dest}, {src}")
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
            Instruction::Inc { src } => {
                write!(f, "inc {src}")
            }
            Instruction::Dec { src } => {
                write!(f, "dec {src}")
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
            Instruction::Mul { src } => {
                write!(f, "mul {src}")
            }
            Instruction::Imul { src } => {
                write!(f, "imul {src}")
            }
            Instruction::Div { src } => {
                write!(f, "div {src}")
            }
            Instruction::Idiv { src } => {
                write!(f, "idiv {src}")
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
            Instruction::MoveByte { repeat } => {
                match repeat {
                    Some(Repeat::WhileClearZeroFlag) => write!(f, "repne ")?,
                    Some(Repeat::WhileSetZeroFlag) => write!(f, "repe ")?,
                    None => {
                        // Nothing to write if no repeat prefix
                    }
                }

                write!(f, "movsb")
            }
            Instruction::MoveWord { repeat } => {
                match repeat {
                    Some(Repeat::WhileClearZeroFlag) => write!(f, "repne ")?,
                    Some(Repeat::WhileSetZeroFlag) => write!(f, "repe ")?,
                    None => {
                        // Nothing to write if no repeat prefix
                    }
                }

                write!(f, "movsw")
            }
            Instruction::CmpByte { repeat } => {
                match repeat {
                    Some(Repeat::WhileClearZeroFlag) => write!(f, "repne ")?,
                    Some(Repeat::WhileSetZeroFlag) => write!(f, "repe ")?,
                    None => {
                        // Nothing to write if no repeat prefix
                    }
                }

                write!(f, "cmpsb")
            }
            Instruction::CmpWord { repeat } => {
                match repeat {
                    Some(Repeat::WhileClearZeroFlag) => write!(f, "repne ")?,
                    Some(Repeat::WhileSetZeroFlag) => write!(f, "repe ")?,
                    None => {
                        // Nothing to write if no repeat prefix
                    }
                }

                write!(f, "cmpsw")
            }
            Instruction::ScanByte { repeat } => {
                match repeat {
                    Some(Repeat::WhileClearZeroFlag) => write!(f, "repne ")?,
                    Some(Repeat::WhileSetZeroFlag) => write!(f, "repe ")?,
                    None => {
                        // Nothing to write if no repeat prefix
                    }
                }

                write!(f, "scasb")
            }
            Instruction::ScanWord { repeat } => {
                match repeat {
                    Some(Repeat::WhileClearZeroFlag) => write!(f, "repne ")?,
                    Some(Repeat::WhileSetZeroFlag) => write!(f, "repe ")?,
                    None => {
                        // Nothing to write if no repeat prefix
                    }
                }

                write!(f, "scasw")
            }
            Instruction::LoadByte { repeat } => {
                match repeat {
                    Some(Repeat::WhileClearZeroFlag) => write!(f, "repne ")?,
                    Some(Repeat::WhileSetZeroFlag) => write!(f, "repe ")?,
                    None => {
                        // Nothing to write if no repeat prefix
                    }
                }

                write!(f, "lodsb")
            }
            Instruction::LoadWord { repeat } => {
                match repeat {
                    Some(Repeat::WhileClearZeroFlag) => write!(f, "repne ")?,
                    Some(Repeat::WhileSetZeroFlag) => write!(f, "repe ")?,
                    None => {
                        // Nothing to write if no repeat prefix
                    }
                }

                write!(f, "lodsw")
            }
            Instruction::StoreByte { repeat } => {
                match repeat {
                    Some(Repeat::WhileClearZeroFlag) => write!(f, "repne ")?,
                    Some(Repeat::WhileSetZeroFlag) => write!(f, "repe ")?,
                    None => {
                        // Nothing to write if no repeat prefix
                    }
                }

                write!(f, "stosb")
            }
            Instruction::StoreWord { repeat } => {
                match repeat {
                    Some(Repeat::WhileClearZeroFlag) => write!(f, "repne ")?,
                    Some(Repeat::WhileSetZeroFlag) => write!(f, "repe ")?,
                    None => {
                        // Nothing to write if no repeat prefix
                    }
                }

                write!(f, "stosw")
            }
            Instruction::Call { dest } => {
                write!(f, "call {dest}")
            }

            Instruction::Jump { dest } => {
                write!(f, "jmp {dest}")
            }

            Instruction::ReturnWithOffset { offset } => {
                write!(f, "ret {offset:#x}")
            }

            Instruction::Return => {
                write!(f, "ret")
            }
            Instruction::JumpEqual { offset, label } => {
                if let Some(label) = label {
                    write!(f, "je {label}")
                } else {
                    write!(f, "je {offset}")
                }
            }
            Instruction::JumpLessThan { offset, label } => {
                if let Some(label) = label {
                    write!(f, "jl {label}")
                } else {
                    write!(f, "jl {offset}")
                }
            }
            Instruction::JumpLessThanEqual { offset, label } => {
                if let Some(label) = label {
                    write!(f, "jle {label}")
                } else {
                    write!(f, "jle {offset}")
                }
            }
            Instruction::JumpBelow { offset, label } => {
                if let Some(label) = label {
                    write!(f, "jb {label}")
                } else {
                    write!(f, "jb {offset}")
                }
            }
            Instruction::JumpBelowEqual { offset, label } => {
                if let Some(label) = label {
                    write!(f, "jbe {label}")
                } else {
                    write!(f, "jbe {offset}")
                }
            }
            Instruction::JumpParityEven { offset, label } => {
                if let Some(label) = label {
                    write!(f, "jp {label}")
                } else {
                    write!(f, "jp {offset}")
                }
            }
            Instruction::JumpParityOdd { offset, label } => {
                if let Some(label) = label {
                    write!(f, "jnp {label}")
                } else {
                    write!(f, "jnp {offset}")
                }
            }
            Instruction::JumpNotOverflow { offset, label } => {
                if let Some(label) = label {
                    write!(f, "jno {label}")
                } else {
                    write!(f, "jno {offset}")
                }
            }
            Instruction::JumpNotSign { offset, label } => {
                if let Some(label) = label {
                    write!(f, "jns {label}")
                } else {
                    write!(f, "jns {offset}")
                }
            }
            Instruction::JumpOverflow { offset, label } => {
                if let Some(label) = label {
                    write!(f, "jo {label}")
                } else {
                    write!(f, "jo {offset}")
                }
            }
            Instruction::JumpSign { offset, label } => {
                if let Some(label) = label {
                    write!(f, "js {label}")
                } else {
                    write!(f, "js {offset}")
                }
            }
            Instruction::JumpNotEqual { offset, label } => {
                if let Some(label) = label {
                    write!(f, "jne {label}")
                } else {
                    write!(f, "jne {offset}")
                }
            }
            Instruction::JumpNotLessThan { offset, label } => {
                if let Some(label) = label {
                    write!(f, "jnl {label}")
                } else {
                    write!(f, "jnl {offset}")
                }
            }
            Instruction::JumpNotLessThanEqual { offset, label } => {
                if let Some(label) = label {
                    write!(f, "jnle {label}")
                } else {
                    write!(f, "jnle {offset}")
                }
            }
            Instruction::JumpNotBelow { offset, label } => {
                if let Some(label) = label {
                    write!(f, "jnb {label}")
                } else {
                    write!(f, "jnb {offset}")
                }
            }
            Instruction::JumpNotBelowEqual { offset, label } => {
                if let Some(label) = label {
                    write!(f, "jnbe {label}")
                } else {
                    write!(f, "jnbe {offset}")
                }
            }
            Instruction::Loop { offset, label } => {
                if let Some(label) = label {
                    write!(f, "loop {label}")
                } else {
                    write!(f, "loop {offset}")
                }
            }
            Instruction::LoopWhileZero { offset, label } => {
                if let Some(label) = label {
                    write!(f, "loope {label}")
                } else {
                    write!(f, "loope {offset}")
                }
            }
            Instruction::LoopWhileNotZero { offset, label } => {
                if let Some(label) = label {
                    write!(f, "loopne {label}")
                } else {
                    write!(f, "loopne {offset}")
                }
            }
            Instruction::JumpCxZero { offset, label } => {
                if let Some(label) = label {
                    write!(f, "jcxz {label}")
                } else {
                    write!(f, "jcxz {offset}")
                }
            }
            Instruction::Interrupt { vector } => {
                write!(f, "int {vector:#x}")
            }
            Instruction::InterruptOnOverflow => {
                write!(f, "into")
            }
            Instruction::InterruptReturn => {
                write!(f, "iret")
            }
            Instruction::ClearCarry => {
                write!(f, "clc")
            }
            Instruction::ComplementCarry => {
                write!(f, "cmc")
            }
            Instruction::SetCarry => {
                write!(f, "stc")
            }
            Instruction::ClearDirection => {
                write!(f, "cld")
            }
            Instruction::SetDirection => {
                write!(f, "std")
            }
            Instruction::ClearInterrupt => {
                write!(f, "cli")
            }
            Instruction::SetInterrupt => {
                write!(f, "sti")
            }
            Instruction::Halt => {
                write!(f, "hlt")
            }
            Instruction::Wait => {
                write!(f, "wait")
            }
            Instruction::Lock => {
                write!(f, "lock ")
            }
            Instruction::Label { name } => {
                write!(f, "{name}:")
            }
        }
    }
}
