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
                dest: Operand::Register(reg),
                src: Operand::Immediate(imm),
            } => {
                let (main_reg, piece) = reg.as_sub_register();

                let value = match piece {
                    SubRegister::Full => *imm as u16,
                    SubRegister::Low => {
                        let imm = *imm as u16;
                        assert!(imm <= 0xff);
                        let mut old = self.registers.regs[main_reg as usize];
                        old &= 0xff00;
                        old | imm
                    }
                    SubRegister::High => {
                        let imm = *imm as u16;
                        assert!(imm <= 0xff);
                        let mut old = self.registers.regs[main_reg as usize];
                        old &= 0x00ff;
                        old | (imm << 8)
                    }
                };

                // Write the immediate to the register file
                self.registers.regs[main_reg as usize] = value;
            }
            Instruction::Mov {
                dest: Operand::Register(dest),
                src: Operand::Register(src),
            } => {
                let (dest_reg, dest_piece) = dest.as_sub_register();
                let (src_reg, src_piece) = src.as_sub_register();

                let old = self.registers.regs[src_reg as usize];
                let imm = match src_piece {
                    SubRegister::Full => old,
                    SubRegister::Low => old & 0xff,
                    SubRegister::High => old & (0xff00) >> 8,
                };

                let value = match dest_piece {
                    SubRegister::Full => imm as u16,
                    SubRegister::Low => {
                        let mut old = self.registers.regs[dest_reg as usize];
                        old &= 0xff00;
                        old | imm
                    }
                    SubRegister::High => {
                        let mut old = self.registers.regs[dest_reg as usize];
                        old &= 0x00ff;
                        old | (imm << 8)
                    }
                };

                // Copy the source register to the dest register
                self.registers.regs[dest_reg as usize] = value;
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
            _ => panic!("Cannot execute: {instr:?}"),
        }
    }
}
