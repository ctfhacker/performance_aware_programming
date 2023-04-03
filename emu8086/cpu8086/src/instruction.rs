//! 8086 Instruction

use crate::memory_operand::MemoryOperand;
use crate::register::{Register, SegmentRegister};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operand {
    // A register operand
    Register(Register),

    // A memory operand
    Memory(MemoryOperand),

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
                        write!(f, "{size} ")?;
                    }

                    if let Some(segment) = memory.segment {
                        write!(f, "{segment}:")?;
                    }

                    write!(f, "[{address:#x}]")?;
                    return Ok(());
                }

                // Start the memory bracket
                if let Some(size) = memory.size {
                    write!(f, "{size} ")?;
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Repeat {
    /// Repeat/loop while zero flag is clear
    WhileClearZeroFlag,

    /// Repeat/loop while zero flag is set
    WhileSetZeroFlag,
}

/// An 8086 decoded instruction
#[derive(Debug, PartialEq, Eq)]
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
    Cmp { left: Operand, right: Operand },

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
    JumpEqual { offset: i8 },

    /// Jump on less/not greater or equal
    JumpLessThan { offset: i8 },

    /// Jump on lessor or equal/not greater
    JumpLessThanEqual { offset: i8 },

    /// Jump on below/not above or equal
    JumpBelow { offset: i8 },

    /// Jump on below or equal/not above
    JumpBelowEqual { offset: i8 },

    /// Jump on parity/parity even
    JumpParityEven { offset: i8 },

    /// Jump on overflow
    JumpOverflow { offset: i8 },

    /// Jump on sign
    JumpSign { offset: i8 },

    /// Jump on not parity/parity odd
    JumpParityOdd { offset: i8 },

    /// Jump on not equal/not zero
    JumpNotEqual { offset: i8 },

    /// Jump on not less/greater or equal
    JumpNotLessThan { offset: i8 },

    /// Jump on not less or equal/greater
    JumpNotLessThanEqual { offset: i8 },

    /// Jump on not below/above or equal
    JumpNotBelow { offset: i8 },

    /// Jump on not below or equal/above
    JumpNotBelowEqual { offset: i8 },

    /// Jump on not overflow
    JumpNotOverflow { offset: i8 },

    /// Jump on not sign
    JumpNotSign { offset: i8 },

    /// Loop CX times
    Loop { offset: i8 },

    /// Loop while zero
    LoopWhileZero { offset: i8 },

    /// Loop while not zero
    LoopWhileNotZero { offset: i8 },

    /// Jump on CX zero
    JumpCxZero { offset: i8 },

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
    // A jump label
    // Label { name: String },
}

/*
impl From<Operand> for AvxOperand {
    fn from(op: Operand) -> AvxOperand {
        match op {
            Operand::Register(dest) => AvxOperand::Zmm(Zmm(dest.as_zmm())),
            Operand::Immediate(imm) => AvxOperand::Immediate(imm),
            _ => unimplemented!("{op:?}"),
        }
    }
}

impl From<Operand> for Zmm {
    fn from(op: Operand) -> Zmm {
        match op {
            Operand::Register(dest) => Zmm(dest.as_zmm()),
            _ => unimplemented!("Zmm {op:?}"),
        }
    }
}

impl From<Instruction> for JitIL {
    fn from(instr: Instruction) -> JitIL {
        match instr {
            Instruction::Mov { dest, src } => JitIL::Mov {
                dest: dest.into(),
                src: src.into(),
            },
            Instruction::Sub { dest, src } => JitIL::Sub {
                dest: dest.into(),
                op1: dest.into(),
                op2: src.into(),
            },
            Instruction::Add { dest, src } => JitIL::Add {
                dest: dest.into(),
                op1: dest.into(),
                op2: src.into(),
            },
            Instruction::Cmp { left, right } => JitIL::Cmp {
                k: Zmm(2),
                left: left.into(),
                right: right.into(),
                op: CmpOp::Equal,
            },
            _ => unimplemented!("Impl JitIL for {instr:x?}"),
        }
    }
}
*/

impl std::fmt::Display for Instruction {
    #[allow(clippy::too_many_lines)]
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
            Instruction::Cmp {
                left: dest,
                right: src,
            } => {
                write!(f, "cmp {dest}, {src}")
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
            Instruction::JumpEqual { offset } => {
                let mut op = "";
                if *offset >= 0 {
                    op = "+";
                }
                write!(f, "je ${op}{offset}")
            }
            Instruction::JumpLessThan { offset } => {
                let mut op = "";
                if *offset >= 0 {
                    op = "+";
                }
                write!(f, "jl ${op}{offset}")
            }
            Instruction::JumpLessThanEqual { offset } => {
                let mut op = "";
                if *offset >= 0 {
                    op = "+";
                }
                write!(f, "jle ${op}{offset}")
            }
            Instruction::JumpBelow { offset } => {
                let mut op = "";
                if *offset >= 0 {
                    op = "+";
                }
                write!(f, "jb ${op}{offset}")
            }
            Instruction::JumpBelowEqual { offset } => {
                let mut op = "";
                if *offset >= 0 {
                    op = "+";
                }
                write!(f, "jbe ${op}{offset}")
            }
            Instruction::JumpParityEven { offset } => {
                let mut op = "";
                if *offset >= 0 {
                    op = "+";
                }
                write!(f, "jp ${op}{offset}")
            }
            Instruction::JumpParityOdd { offset } => {
                let mut op = "";
                if *offset >= 0 {
                    op = "+";
                }
                write!(f, "jnp ${op}{offset}")
            }
            Instruction::JumpNotOverflow { offset } => {
                let mut op = "";
                if *offset >= 0 {
                    op = "+";
                }
                write!(f, "jno ${op}{offset}")
            }
            Instruction::JumpNotSign { offset } => {
                let mut op = "";
                if *offset >= 0 {
                    op = "+";
                }
                write!(f, "jns ${op}{offset}")
            }
            Instruction::JumpOverflow { offset } => {
                let mut op = "";
                if *offset >= 0 {
                    op = "+";
                }
                write!(f, "jo ${op}{offset}")
            }
            Instruction::JumpSign { offset } => {
                let mut op = "";
                if *offset >= 0 {
                    op = "+";
                }
                write!(f, "js ${op}{offset}")
            }
            Instruction::JumpNotEqual { offset } => {
                let mut op = "";
                if *offset >= 0 {
                    op = "+";
                }
                write!(f, "jne ${op}{offset}")
            }
            Instruction::JumpNotLessThan { offset } => {
                let mut op = "";
                if *offset >= 0 {
                    op = "+";
                }
                write!(f, "jnl ${op}{offset}")
            }
            Instruction::JumpNotLessThanEqual { offset } => {
                let mut op = "";
                if *offset >= 0 {
                    op = "+";
                }
                write!(f, "jnle ${op}{offset}")
            }
            Instruction::JumpNotBelow { offset } => {
                let mut op = "";
                if *offset >= 0 {
                    op = "+";
                }
                write!(f, "jnb ${op}{offset}")
            }
            Instruction::JumpNotBelowEqual { offset } => {
                let mut op = "";
                if *offset >= 0 {
                    op = "+";
                }
                write!(f, "jnbe ${op}{offset}")
            }
            Instruction::Loop { offset } => {
                let mut op = "";
                if *offset >= 0 {
                    op = "+";
                }
                write!(f, "loop ${op}{offset}")
            }
            Instruction::LoopWhileZero { offset } => {
                let mut op = "";
                if *offset >= 0 {
                    op = "+";
                }
                write!(f, "loope ${op}{offset}")
            }
            Instruction::LoopWhileNotZero { offset } => {
                let mut op = "";
                if *offset >= 0 {
                    op = "+";
                }
                write!(f, "loopne ${op}{offset}")
            }
            Instruction::JumpCxZero { offset } => {
                let mut op = "";
                if *offset >= 0 {
                    op = "+";
                }
                write!(f, "jcxz ${op}{offset}")
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
            } // Instruction::Label { name } => { write!(f, "{name}:") }
        }
    }
}

/*
impl Instruction {
    fn _name(&self) -> &'static str {
        match self {
            Instruction::Mov { dest, src } => "mov",
            Instruction::Push { src } => "push",
            Instruction::Pop { src } => "pop",
            Instruction::Xchg { left, right } => "xchg",
            Instruction::Nop => "nop",
            Instruction::In { dest, src } => "in",
            Instruction::Out { dest, src } => "out",
            Instruction::Xlat => "xlat",
            Instruction::Lea { dest, src } => "lea",
            Instruction::Lds { dest, src } => "lds",
            Instruction::Les { dest, src } => "les",
            Instruction::Lahf => "lahf",
            Instruction::Sahf => "sahf",
            Instruction::Pushf => "pushf",
            Instruction::Popf => "popf",
            Instruction::Add { dest, src } => "add",
            Instruction::And { dest, src } => "and",
            Instruction::Test { dest, src } => "test",
            Instruction::Or { dest, src } => "or",
            Instruction::Xor { dest, src } => "xor",
            Instruction::Adc { dest, src } => "adc",
            Instruction::Inc { src } => "inc",
            Instruction::Dec { src } => "dec",
            Instruction::Aaa => "aaa",
            Instruction::Daa => "daa",
            Instruction::Aas => "aas",
            Instruction::Das => "das",
            Instruction::Aam => "aam",
            Instruction::Aad => "aad",
            Instruction::Sub { dest, src } => "sub",
            Instruction::Sbb { dest, src } => "sbb",
            Instruction::Mul { src } => "mul",
            Instruction::Imul { src } => "imul",
            Instruction::Div { src } => "div",
            Instruction::Idiv { src } => "idiv",
            Instruction::Cmp { dest, src } => "cmp",
            Instruction::Neg { src } => "neg",
            Instruction::Cbw => "cbw",
            Instruction::Cwd => "cwd",
            Instruction::Not { src } => "not",
            Instruction::Shl { src, count } => "shl",
            Instruction::Sar { src, count } => "sar",
            Instruction::Shr { src, count } => "shr",
            Instruction::Rol { src, count } => "rol",
            Instruction::Ror { src, count } => "ror",
            Instruction::Rcl { src, count } => "rcl",
            Instruction::Rcr { src, count } => "rcr",
            Instruction::MoveByte { repeat } => match repeat {
                Some(Repeat::WhileClearZeroFlag) => "repne movsb",
                Some(Repeat::WhileSetZeroFlag) => "repe movsb",
                None => "movsb",
            },
            Instruction::MoveWord { repeat } => match repeat {
                Some(Repeat::WhileClearZeroFlag) => "movsw repne",
                Some(Repeat::WhileSetZeroFlag) => "movsw repe ",
                None => "movsw",
            },
            Instruction::CmpByte { repeat } => match repeat {
                Some(Repeat::WhileClearZeroFlag) => "repne",
                Some(Repeat::WhileSetZeroFlag) => "repe",
                None => "cmpsb",
            },
            Instruction::CmpWord { repeat } => match repeat {
                Some(Repeat::WhileClearZeroFlag) => "repne cmpsw",
                Some(Repeat::WhileSetZeroFlag) => "repe cmpsw",
                None => "cmpsw",
            },
            Instruction::ScanByte { repeat } => match repeat {
                Some(Repeat::WhileClearZeroFlag) => "repne scasb",
                Some(Repeat::WhileSetZeroFlag) => "repe scasb",
                None => "scasb",
            },
            Instruction::ScanWord { repeat } => match repeat {
                Some(Repeat::WhileClearZeroFlag) => "repne scasw",
                Some(Repeat::WhileSetZeroFlag) => "repe scasw",
                None => "scasw",
            },
            Instruction::LoadByte { repeat } => match repeat {
                Some(Repeat::WhileClearZeroFlag) => "repne lodsb",
                Some(Repeat::WhileSetZeroFlag) => "repe lodsb",
                None => "lodsb",
            },
            Instruction::LoadWord { repeat } => match repeat {
                Some(Repeat::WhileClearZeroFlag) => "repne lodsw",
                Some(Repeat::WhileSetZeroFlag) => "repe lodsw",
                None => "lodsw",
            },
            Instruction::StoreByte { repeat } => match repeat {
                Some(Repeat::WhileClearZeroFlag) => "repne stosb",
                Some(Repeat::WhileSetZeroFlag) => "repe stosb",
                None => "stosb",
            },
            Instruction::StoreWord { repeat } => match repeat {
                Some(Repeat::WhileClearZeroFlag) => "repne stosw",
                Some(Repeat::WhileSetZeroFlag) => "repe stosw",
                None => "stosw",
            },
            Instruction::Call { dest } => "call",
            Instruction::Jump { dest } => "jmp",
            Instruction::ReturnWithOffset { offset } => "ret",
            Instruction::Return => "ret",
            Instruction::JumpEqual { offset, label } => "je",
            Instruction::JumpLessThan { offset, label } => "jl",
            Instruction::JumpLessThanEqual { offset, label } => "jle",
            Instruction::JumpBelow { offset, label } => "jb",
            Instruction::JumpBelowEqual { offset, label } => "jbe",
            Instruction::JumpParityEven { offset, label } => "jp",
            Instruction::JumpParityOdd { offset, label } => "jnp",
            Instruction::JumpNotOverflow { offset, label } => "jno",
            Instruction::JumpNotSign { offset, label } => "jns",
            Instruction::JumpOverflow { offset, label } => "jo",
            Instruction::JumpSign { offset, label } => "js",
            Instruction::JumpNotEqual { offset, label } => "jne",
            Instruction::JumpNotLessThan { offset, label } => "jnl",
            Instruction::JumpNotLessThanEqual { offset, label } => "jnle",
            Instruction::JumpNotBelow { offset, label } => "jnb",
            Instruction::JumpNotBelowEqual { offset, label } => "jnbe",
            Instruction::Loop { offset, label } => "loop",
            Instruction::LoopWhileZero { offset, label } => "loope",
            Instruction::LoopWhileNotZero { offset, label } => "loopne",
            Instruction::JumpCxZero { offset, label } => "jnxz",
            Instruction::Interrupt { vector } => "int",
            Instruction::InterruptOnOverflow => "into",
            Instruction::InterruptReturn => "iret",
            Instruction::ClearCarry => "clc",
            Instruction::ComplementCarry => "cmc",
            Instruction::SetCarry => "stc",
            Instruction::ClearDirection => "cld",
            Instruction::SetDirection => "std",
            Instruction::ClearInterrupt => "cli",
            Instruction::SetInterrupt => "sti",
            Instruction::Halt => "hlt",
            Instruction::Wait => "wait",
            Instruction::Lock => "lock",
            // Instruction::Label { name } => "label",
        }
    }
}
*/
