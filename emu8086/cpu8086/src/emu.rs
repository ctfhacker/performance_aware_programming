//! An 8086 emulator

use anyhow::Result;

use std::path::Path;

use crate::const_checks::{is_valid_address_size, If, True};
use crate::flags::EFlags;
use crate::instruction::{Instruction, Operand};
use crate::memory::Memory;
use crate::register::{Register, SegmentRegister, SubRegister};

pub struct Emulator<const MEMORY_SIZE: usize> {
    /// The memory in this emulator
    pub memory: Memory<MEMORY_SIZE>,

    /// Register state of the emulator
    pub registers: RegisterState,

    /// Segment registers
    pub segments: [u16; std::mem::variant_count::<SegmentRegister>()],
}

/// The register state of the emulator
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct RegisterState {
    regs: [u16; 10],
}

macro_rules! impl_reg {
    ($reg:ident, $func:ident, $func_mut:ident) => {
        impl RegisterState {
            /// Get this register from the register file
            pub fn $func(&self) -> u16 {
                self.regs[Register::$reg as usize]
            }

            /// Get a mutable reference to this register
            pub fn $func_mut(&mut self) -> &mut u16 {
                &mut self.regs[Register::$reg as usize]
            }
        }

        impl<const MEMORY_SIZE: usize> Emulator<MEMORY_SIZE>
        where
            If<{ is_valid_address_size(MEMORY_SIZE) }>: True,
        {
            /// Get this register from the register file
            pub fn $func(&self) -> u16 {
                self.registers.$func()
            }

            /// Get a mutable reference to this register
            pub fn $func_mut(&mut self) -> &mut u16 {
                self.registers.$func_mut()
            }
        }
    };
}

macro_rules! impl_flag {
    ($flag:ident, $func:ident, $set_func:ident) => {
        impl RegisterState {
            /// Get this register from the register file
            pub fn $func(&self) -> bool {
                self.regs[Register::Flags as usize] & EFlags::$flag as u16 > 0
            }

            /// Get this register from the register file
            pub fn $set_func(&mut self) {
                self.regs[Register::Flags as usize] |= EFlags::$flag as u16;
            }
        }

        impl<const MEMORY_SIZE: usize> Emulator<MEMORY_SIZE>
        where
            If<{ is_valid_address_size(MEMORY_SIZE) }>: True,
        {
            /// Get this flag from the flags register
            pub fn $func(&self) -> bool {
                self.registers.$func()
            }
        }
    };
}

impl_reg!(Ip, ip, ip_mut);
impl_reg!(Flags, flags, flags_mut);
impl_reg!(Ax, ax, ax_mut);
impl_reg!(Bx, bx, bx_mut);
impl_reg!(Cx, cx, cx_mut);
impl_reg!(Dx, dx, dx_mut);
impl_reg!(Si, si, si_mut);
impl_reg!(Di, di, di_mut);
impl_reg!(Bp, bp, bp_mut);
impl_reg!(Sp, sp, sp_mut);

impl_flag!(Zero, zero_flag, set_zero_flag);
impl_flag!(Sign, sign_flag, set_sign_flag);
impl_flag!(Carry, carry_flag, set_carry_flag);
impl_flag!(Parity, parity_flag, set_parity_flag);
impl_flag!(Overflow, overflow_flag, set_overflow_flag);
impl_flag!(Auxillary, auxillary_carry_flag, set_auxillary_carry_flag);

impl<const MEMORY_SIZE: usize> Emulator<MEMORY_SIZE>
where
    If<{ is_valid_address_size(MEMORY_SIZE) }>: True,
{
    /// Create an emulator with blank memory
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            registers: RegisterState::default(),
            segments: [0; std::mem::variant_count::<SegmentRegister>()],
        }
    }

    /// Create an emulator
    pub fn with_memory(path: &Path) -> Result<Self> {
        Ok(Self {
            memory: Memory::from_file(path)?,
            registers: RegisterState::default(),
            segments: [0; std::mem::variant_count::<SegmentRegister>()],
        })
    }

    /// Get the register value in the given [`Register`]
    pub fn get_register_value(&self, reg: &Register) -> u16 {
        // Get the sub register for the given register
        let (reg_reg, reg_piece) = reg.as_sub_register();

        // Extract the register value
        let reg = self.registers.regs[reg_reg as usize];

        // Mask the register valueu based on the subregister
        match reg_piece {
            SubRegister::Full => reg,
            SubRegister::Low => reg & 0xff,
            SubRegister::High => (reg & (0xff00)) >> 8,
        }
    }

    /// Set the given [`Register`] to the given value
    pub fn set_register_value(&mut self, dest: &Register, imm: u16) {
        // Get the sub register for the destination
        let (dest_reg, dest_piece) = dest.as_sub_register();

        // Get the destination value based on the current destination register
        let value = match dest_piece {
            SubRegister::Full => imm as u16,
            SubRegister::Low => {
                assert!(imm & 0x100 == 0, "Cannot write {imm:#x} into {dest:?}");
                let mut old = self.registers.regs[dest_reg as usize];
                old &= 0xff00;
                old | (imm & 0xff)
            }
            SubRegister::High => {
                assert!(imm & 0x100 == 0, "Cannot write {imm:#x} into {dest:?}");
                let mut old = self.registers.regs[dest_reg as usize];
                old &= 0x00ff;
                old | (imm << 8)
            }
        };

        // Copy the source register to the dest register
        self.registers.regs[dest_reg as usize] = value;
    }

    /// Set the status flags based on the given val
    pub fn set_status_flags(&mut self, val: u16, arg1: u16, arg2: u16) {
        let mut new_flags = 0;
        if val == 0 {
            new_flags |= EFlags::Zero as u16;
        }
        if val & 0x8000 > 0 {
            new_flags |= EFlags::Sign as u16;
        }
        if arg1 & 0x8000 > 0 && arg2 & 0x8000 > 0 && val & 0x8000 == 0 {
            new_flags |= EFlags::Overflow as u16;
        }
        if arg1 & 0x8000 == 0 && arg2 & 0x8000 == 0 && val & 0x8000 > 0 {
            new_flags |= EFlags::Carry as u16;
        }
        if (val & 0xff).count_ones() % 2 == 0 {
            new_flags |= EFlags::Parity as u16;
        }
        if arg1 & 0x80 == 0 && arg2 & 0x80 == 0 && val & 0x80 > 0 {
            new_flags |= EFlags::Auxillary as u16;
        }

        self.registers.regs[Register::Flags as usize] = new_flags;
    }

    /// Print the CPU state
    pub fn print_context(&self) {
        // Get the current flags reg
        let flags = self.flags();

        // Create the FLAGS string
        let mut eflags = String::new();
        for (flag, ch) in [
            (EFlags::Carry, "C"),
            (EFlags::Parity, "P"),
            (EFlags::Auxillary, "A"),
            (EFlags::Zero, "Z"),
            (EFlags::Sign, "S"),
            (EFlags::Overflow, "O"),
        ] {
            if flags & flag as u16 > 0 {
                eflags.push_str(ch);
            }
        }

        // Pretty print this core's register state
        use crate::register::Register::*;
        let ax = self.ax();
        let bx = self.bx();
        let cx = self.cx();
        let dx = self.dx();
        let si = self.si();
        let di = self.di();
        let bp = self.bp();
        let sp = self.sp();
        let ip = self.ip();

        let cs = self.segments[SegmentRegister::Cs as usize];
        let ds = self.segments[SegmentRegister::Ds as usize];
        let es = self.segments[SegmentRegister::Es as usize];
        let ss = self.segments[SegmentRegister::Ss as usize];

        // Pretty print this core's register state
        println!("IP: {ip:04x} FLAGS: {flags:04x} {eflags}");
        println!("AX: {ax:04x} BX: {bx:04x} CX: {cx:04x} DX: {dx:04x}");
        println!("SP: {sp:04x} BP: {bp:04x} SI: {si:04x} DI: {di:04x}");
        println!("CS: {cs:04x} DS: {ds:04x} ES: {es:04x} SS: {ss:04x}");
    }

    pub fn execute(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Mov {
                dest: Operand::Register(dest),
                src: Operand::Immediate(imm),
            } => {
                self.set_register_value(dest, *imm as u16);
            }
            Instruction::Mov {
                dest: Operand::Register(dest),
                src: Operand::Register(src),
            } => {
                let src_val = self.get_register_value(src);
                self.set_register_value(dest, src_val);
            }
            Instruction::Mov {
                dest: Operand::SegmentRegister(dest),
                src: Operand::Register(src),
            } => {
                let (main_reg, piece) = src.as_sub_register();
                assert!(
                    piece == SubRegister::Full,
                    "Cannot move sub-register to segment"
                );

                self.segments[*dest as usize] = self.registers.regs[main_reg as usize];
            }
            Instruction::Mov {
                dest: Operand::Register(dest),
                src: Operand::SegmentRegister(src),
            } => {
                let (_main_reg, piece) = dest.as_sub_register();
                assert!(
                    piece == SubRegister::Full,
                    "Cannot move sub-register to segment"
                );

                self.registers.regs[*dest as usize] = self.segments[*src as usize];
            }
            Instruction::Sub {
                dest: Operand::Register(dest),
                src: Operand::Register(src),
            } => {
                let dest_val = self.get_register_value(dest);
                let src_val = self.get_register_value(src);

                let new_val = dest_val.wrapping_sub(src_val);

                // Set the status flags based on the resulting value
                self.set_status_flags(new_val, dest_val, src_val);

                self.set_register_value(dest, new_val);
            }
            Instruction::Sub {
                dest: Operand::Register(dest),
                src,
            } => {
                let dest_val = self.get_register_value(dest);

                let src_val = match src {
                    Operand::Register(src) => self.get_register_value(src),
                    Operand::Immediate(imm) => *imm as u16,
                    _ => unreachable!(),
                };

                let new_val = dest_val.wrapping_sub(src_val);
                self.set_register_value(dest, new_val);
                self.set_status_flags(new_val, dest_val, src_val);
            }
            Instruction::Add {
                dest: Operand::Register(dest),
                src,
            } => {
                let dest_val = self.get_register_value(dest);

                let src_val = match src {
                    Operand::Register(src) => self.get_register_value(src),
                    Operand::Immediate(imm) => *imm as u16,
                    _ => unreachable!(),
                };

                let new_val = dest_val.wrapping_add(src_val);
                self.set_register_value(dest, new_val);
                self.set_status_flags(new_val, dest_val, src_val);
            }
            Instruction::Cmp {
                left: Operand::Register(dest),
                right: Operand::Register(src),
            } => {
                let dest_val = self.get_register_value(dest);
                let src_val = self.get_register_value(src);

                let new_val = dest_val.wrapping_sub(src_val);
                self.set_status_flags(new_val, dest_val, src_val);
            }
            Instruction::JumpNotEqual { offset } => {
                if !self.zero_flag() {
                    let new_ip = self.ip().wrapping_add_signed(*offset as i16 - 2);
                    self.set_register_value(&Register::Ip, new_ip);
                }
            }
            Instruction::JumpEqual { offset } => {
                if self.zero_flag() {
                    let new_ip = self.ip().wrapping_add_signed(*offset as i16 - 2);
                    self.set_register_value(&Register::Ip, new_ip);
                }
            }
            Instruction::JumpBelow { offset } => {
                if self.carry_flag() {
                    let new_ip = self.ip().wrapping_add_signed(*offset as i16 - 2);
                    self.set_register_value(&Register::Ip, new_ip);
                }
            }
            Instruction::LoopWhileNotZero { offset } => {
                let new_cx = self.cx().saturating_sub(1);
                self.set_register_value(&Register::Cx, new_cx);

                if new_cx != 0 {
                    let new_ip = self.ip().wrapping_add_signed(*offset as i16 - 2);
                    self.set_register_value(&Register::Ip, new_ip);
                }
            }
            Instruction::JumpParityEven { offset } => {
                if self.parity_flag() {
                    let new_ip = self.ip().wrapping_add_signed(*offset as i16 - 2);
                    self.set_register_value(&Register::Ip, new_ip);
                }
            }
            _ => panic!("Cannot execute: {instr:?}"),
        }
    }
}
