#![feature(portable_simd)]

mod utils;
use utils::alloc_rwx;

mod il;
pub use il::{CmpOp, JitIL};

mod evex;
pub use evex::{Avx512Instruction, AvxOpcode, AvxOperand, Kmask, Zmm};

use cpu8086::flags::EFlags;
use cpu8086::instruction::{Instruction, Operand};

/// Enum used to identify the zmm register for each 8086 register
/// Example::
/// ax - zmm{Register8086::ax as usize}
/// bx - zmm{Register8086::bx as usize}
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum JitRegister {
    zero,
    ax,
    bx,
    cx,
    dx,
    si,
    di,
    sp,
    bp,
    ip,
    flags,
}

impl JitRegister {
    /// Get the zmm register number this register uses in the JIT
    pub fn as_zmm(self) -> Zmm {
        Zmm(self as u8)
    }
}

#[allow(non_camel_case_types)]
pub enum HostRegister {
    rax,
    rcx,
    rdx,
    rbx,
    rsp,
    rbp,
    rsi,
    rdi,
    r8,
    r9,
    r10,
    r11,
    r12,
    r13,
    r14,
    r15,
}

impl HostRegister {
    /// Get the zmm register number this register uses in the JIT
    pub fn as_zmm(self) -> Zmm {
        Zmm(self as u8)
    }
}

pub struct JitBuffer<const N: usize> {
    /// RWX allocation where the JIT instructions are written
    buffer: *mut u8,

    /// Current offset in `buffer` where new instructions are written
    pub offset: isize,

    /// Scratch registers available to be allocated during the JIT
    scratch_registers: [Zmm; 4],

    /// The current index for allocating a new scratch register
    scratch_registers_index: usize,
}

impl<const N: usize> JitBuffer<N> {
    /// Create a new JIT buffer of size `N`
    pub fn new() -> JitBuffer<N> {
        // Alloc N bytes as RWX for the JIT buffer
        let buffer = alloc_rwx(N);

        // Initialize the JIT buffer to ret's
        unsafe {
            std::ptr::write_bytes(buffer, 0xc3, N);
        }

        // Return the created JitBuffer
        JitBuffer {
            buffer,
            offset: 0,
            scratch_registers: [Zmm(28), Zmm(29), Zmm(30), Zmm(31)],
            scratch_registers_index: 0,
        }
    }

    /// Get the pointer to the JIT buffer
    pub fn buffer(&self) -> *const u8 {
        self.buffer
    }

    /// Get the next available scratch register
    pub fn next_scratch_reg(&mut self) -> Zmm {
        let reg = self.scratch_registers[self.scratch_registers_index];
        self.scratch_registers_index =
            (self.scratch_registers_index + 1) % self.scratch_registers.len();
        reg
    }

    /// Write the given byte into the JIT stream at the current byte offset
    pub fn write_instr<J: Into<JitIL>>(&mut self, instr: J) {
        let instr: JitIL = instr.into();
        self._write_instr(&instr);
    }

    pub fn mov_imm(&mut self, dest: Zmm, imm: i16) {
        // Assemble: mov esi, VAL
        let bytes = [0xbe];
        self.write_bytes(&bytes);
        self.write_bytes(&(imm as u32).to_le_bytes());

        let instr = vpbroadcastw!(dest, rsi);
        let bytes = instr.assemble();
        self.write_bytes(&bytes.as_slice());
    }

    /// Move immediate using the k opmask
    pub fn mov_imm_with_kmask(&mut self, dest: Zmm, imm: i16, kmask: Kmask) {
        // Assemble: mov esi, VAL
        let bytes = [0xbe];
        self.write_bytes(&bytes);
        self.write_bytes(&(imm as u32).to_le_bytes());

        let instr = vpbroadcastw!(dest, rsi, kmask);
        let bytes = instr.assemble();
        self.write_bytes(&bytes.as_slice());
    }

    pub fn mov(&mut self, dest: Zmm, src: Zmm) {
        let bytes = vpmovdqa64!(dest, src).assemble();
        self.write_bytes(&bytes.as_slice());
    }

    /// dest = op1 - op2
    pub fn sub(&mut self, dest: Zmm, op1: Zmm, op2: Zmm) {
        let bytes = vpsubw!(dest, op1, op2).assemble();
        self.write_bytes(&bytes.as_slice());

        // Update status bits based on the result
        self.set_zero_flag(dest);
        self.set_sign_flag(dest);
    }

    /// Set the Zero Flag in the EFLAGS register if `dest` is zero
    pub fn set_zero_flag(&mut self, dest: Zmm) {
        // Fill a ZMM with zero
        let zero_zmm = self.next_scratch_reg();
        self.clear_zmm(zero_zmm);

        let kmask = Kmask(3);
        let tmp_flag_zmm = self.next_scratch_reg();
        self.cmp(Zmm(kmask.0), dest, zero_zmm, CmpOp::Equal);
        self.mov_imm_with_kmask(tmp_flag_zmm, 1 << (EFlags::Zero as usize), kmask);
        let flags = JitRegister::flags.as_zmm();
        self.or_with_kmask(flags, flags, tmp_flag_zmm, kmask);
    }

    /// Set the Sign Flag in the EFLAGS register if `dest` is less than zero
    pub fn set_sign_flag(&mut self, dest: Zmm) {
        // Fill a ZMM with zero
        let zero_zmm = self.next_scratch_reg();
        self.clear_zmm(zero_zmm);

        let kmask = Kmask(3);
        let tmp_flag_zmm = self.next_scratch_reg();
        self.cmp(Zmm(kmask.0), dest, zero_zmm, CmpOp::LessThan);
        self.mov_imm_with_kmask(tmp_flag_zmm, 1 << (EFlags::Sign as usize), kmask);
        let flags = JitRegister::flags.as_zmm();
        self.or_with_kmask(flags, flags, tmp_flag_zmm, kmask);
    }

    /// dest = op1 + op2
    pub fn add(&mut self, dest: Zmm, op1: Zmm, op2: Zmm) {
        let bytes = vpaddw!(dest, op1, op2).assemble();
        self.write_bytes(&bytes.as_slice());
    }

    /// dest = op1 || op2
    pub fn or(&mut self, dest: Zmm, op1: Zmm, op2: Zmm) {
        let bytes = vporw!(dest, op1, op2).assemble();
        self.write_bytes(&bytes.as_slice());
    }

    /// dest = op1 || op2
    pub fn or_with_kmask(&mut self, dest: Zmm, op1: Zmm, op2: Zmm, kmask: Kmask) {
        let bytes = vporw!(dest, op1, op2, kmask).assemble();
        self.write_bytes(&bytes.as_slice());
    }

    /// k = left [`CmpOp`] right
    pub fn cmp(&mut self, k: Zmm, left: Zmm, right: Zmm, op: CmpOp) {
        let bytes = vpcmpw!(k, left, right, op).assemble();
        self.write_bytes(&bytes.as_slice());
    }

    /// Write a `ret` instruction
    pub fn ret(&mut self) {
        self.write_bytes(&[0xc3]);
    }

    pub fn clear_zmm(&mut self, dest: Zmm) {
        let bytes = vpxorq!(dest).assemble();
        self.write_bytes(&bytes.as_slice());
    }

    /// Get the register holding the value in the operand
    /// AvxOperand::Zmm(zmm) -> Return the given register
    /// AvxOperand::Immediate(imm) ->
    ///    Broadcast this immediate to a scratch register and return this register
    pub fn operand_to_register(&mut self, operand: AvxOperand) -> Zmm {
        // Get the op2 based on
        match operand {
            AvxOperand::Zmm(zmm) => zmm,
            AvxOperand::Immediate(imm) => {
                // If there is an immediate operand, broadcast it to the scratch register
                // and use the scratch register to do this operation
                let new_src = self.next_scratch_reg();
                self.mov_imm(new_src, imm);
                new_src
            }
        }
    }

    /// Internal function to write the given [`JitIL`] instruction into the JIT stream
    fn _write_instr(&mut self, instr: &JitIL) {
        match instr {
            JitIL::Mov { dest, src } => {
                match src {
                    AvxOperand::Immediate(imm) => {
                        // Broadcast the given immediate to
                        self.mov_imm(*dest, *imm);
                    }
                    AvxOperand::Zmm(src) => {
                        self.mov(*dest, *src);
                    }
                }
            }
            JitIL::Sub { dest, op1, op2 } => {
                let op2 = self.operand_to_register(*op2);
                self.sub(*dest, *op1, op2);
            }
            JitIL::Add { dest, op1, op2 } => {
                let op2 = self.operand_to_register(*op2);
                self.add(*dest, *op1, op2);
            }
            JitIL::Cmp { k, left, right, op } => {
                let left = self.operand_to_register(*left);
                let right = self.operand_to_register(*right);
                self.cmp(*k, left, right, *op);
            }
        };
    }

    /// Write the given bytes into the JIT stream at the current byte offset
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        assert!(
            self.offset + bytes.len() as isize <= N as isize,
            "OOB write of JIT buffer"
        );

        // Write the given bytes into the JIT buffer at the current offset
        unsafe {
            let curr_buffer = self.buffer.offset(self.offset);
            let data = std::slice::from_raw_parts_mut(curr_buffer, bytes.len());
            data.copy_from_slice(bytes);
        }

        // Update the offset
        self.offset += bytes.len() as isize;
    }

    pub fn print_disassembly(&self, n: usize) {
        use iced_x86::{Decoder, DecoderOptions, FastFormatter, Instruction};

        let data = unsafe { std::slice::from_raw_parts_mut(self.buffer, N) };
        let mut output = String::new();
        let mut offset = 0;

        for _ in 0..n {
            let mut decoder = Decoder::new(64, &data[offset..], DecoderOptions::NONE);

            let mut instr = Instruction::default();
            decoder.decode_out(&mut instr);

            // Create the formatter
            let mut formatter = FastFormatter::new();
            formatter.options_mut().set_uppercase_hex(false);
            formatter.options_mut().set_use_hex_prefix(true);
            formatter.options_mut().set_always_show_memory_size(true);
            formatter
                .options_mut()
                .set_space_after_operand_separator(true);
            // formatter.options_mut().set_rip_relative_addresses(true);

            // Format the instruction into the output
            formatter.format(&instr, &mut output);
            println!("{output}");

            output.clear();

            // Increase the offset by the instruction length
            offset += instr.len();
        }
    }

    /// Get the disassembly instruction at the given offset in the JIT buffer
    pub fn get_disassembly_between(&self, start: isize, end: isize) -> Vec<String> {
        assert!(start < end);

        use iced_x86::{Decoder, DecoderOptions, FastFormatter, Instruction};

        let mut offset = start;
        let mut result = Vec::new();

        while offset < end {
            let data = unsafe {
                std::slice::from_raw_parts_mut(self.buffer.offset(offset), N - offset as usize)
            };
            let mut output = String::new();
            let mut decoder = Decoder::new(64, &data, DecoderOptions::NONE);

            let mut instr = Instruction::default();
            decoder.decode_out(&mut instr);

            // Create the formatter
            let mut formatter = FastFormatter::new();
            formatter.options_mut().set_uppercase_hex(false);
            formatter.options_mut().set_use_hex_prefix(true);
            formatter.options_mut().set_always_show_memory_size(true);
            formatter
                .options_mut()
                .set_space_after_operand_separator(true);
            // formatter.options_mut().set_rip_relative_addresses(true);

            // Format the instruction into the output
            formatter.format(&instr, &mut output);

            // Increase the offset past the decoded instruction
            offset += instr.len() as isize;

            result.push(output)
        }

        result
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    #[test]
    fn test_evex() {
        for (instr, needed) in [
            (vpsubw!(Zmm(1), Zmm(2), Zmm(1)),        vec![0x62, 0xf1, 0x6d, 0x48, 0xf9, 0xc9]),
            (vpaddw!(Zmm(1), Zmm(2), Zmm(1)),        vec![0x62, 0xf1, 0x6d, 0x48, 0xfd, 0xc9]),
            (vpcmpw!(Zmm(1), Zmm(8), Zmm(7), CmpOp::Equal), vec![0x62, 0xf3, 0xbd, 0x48, 0x3f, 0xcf, 0x00]),
            (vpbroadcastw!(Zmm(1), rsi),  vec![0x62, 0xf2, 0x7d, 0x48, 0x7b, 0xce]),
            (vpbroadcastw!(Zmm(1), rax),  vec![0x62, 0xf2, 0x7d, 0x48, 0x7b, 0xc8]),
            (vpbroadcastw!(Zmm(1), rcx),  vec![0x62, 0xf2, 0x7d, 0x48, 0x7b, 0xc9]),
            (vpmovdqa64!(Zmm(1), Zmm(2)), vec![0x62, 0xf1, 0xfd, 0x48, 0x6f, 0xca]),
            (vpmovdqa64!(Zmm(2), Zmm(1)), vec![0x62, 0xf1, 0xfd, 0x48, 0x6f, 0xd1]),
        ] {
            let bytes = instr.assemble();
            let instr2 = instr.clone();
            assert!(
                bytes.as_slice() == needed,
                "{instr2:?} | {:x?} {needed:x?}",
                bytes.as_slice(),
            );
        }
    }
}
