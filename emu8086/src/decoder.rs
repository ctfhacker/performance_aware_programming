use crate::instruction::{Instruction, Mod, Operand, Reg, Rm, Wide};
use crate::memory::Memory;
use crate::register::Register;

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecoderError {
    #[error("Attempted to parse an unknown instruction: {0:#x}")]
    UnknownInstruction(u8),

    #[error("Middle 3 bits of second byte of Imm to Reg/Mem instruction (0b1100011x) are invalid: {0:#x} {1:#x}")]
    InvalidImmediateToRegisterMemoryByte2(u8, u8),
}

/// Decode a stream of bytes and return the decoded `Instruction`s
pub fn decode_stream(mut input: &[u8]) -> Result<Vec<Instruction>> {
    let mut res = Vec::new();

    while !input.is_empty() {
        let instr = match input[0] {
            // Register/memory to/from register
            0b1000_1000..=0b1000_1011 => {
                // Parse the bit fields
                let w = input[0] & 1;
                let d = (input[0] >> 1) & 1 > 0;
                let rm = (input[1] >> 0) & 0b111;
                let reg = (input[1] >> 3) & 0b111;
                let mod_ = (input[1] >> 6) & 0b11;

                eprintln!(
                    "{:#x} -> w {w} d {d} rm {rm:03b} reg {reg:03b} mod {mod_:02b}",
                    input[0]
                );

                let (reg, rm, size) = parse_mov_instr(input, Mod(mod_), Reg(reg), Wide(w), Rm(rm))?;

                // Update the input bytes
                input = &input[size..];

                // Align the src/dest to the proper position based on the `d` flag
                if d {
                    Instruction::Mov { dest: reg, src: rm }
                } else {
                    Instruction::Mov { dest: rm, src: reg }
                }
            }
            // Immediate to register/memory
            0b1100_0110..=0b1100_0111 => {
                // Parse the bit fields
                let wide = input[0] & 1;

                // Sanity check the middle 3 bits of the second byte for this instruction
                // are zero. Otherwise, it is an error
                if input[1] >> 3 & 0b111 != 0 {
                    return Err(DecoderError::InvalidImmediateToRegisterMemoryByte2(
                        input[0], input[1],
                    )
                    .into());
                }

                let mod_ = input[1] >> 6 & 0b11;
                let rm = input[1] & 0b111;

                let (_, rm, mut size) =
                    parse_mov_instr(input, Mod(mod_), Reg(0), Wide(wide), Rm(rm))?;

                eprintln!("ImmToRegMem: mod {mod_:b} rm {rm:?} size {size}");

                // Continue parsing the immediate after the parsed size is there was
                // a displacement or not
                let mut imm = input[size] as u16;
                size += 1;
                if wide > 0 {
                    imm |= (input[size] as u16) << 8;
                    size += 1;
                }

                // Update the input bytes
                input = &input[size..];

                Instruction::Mov {
                    dest: rm,
                    src: Operand::Immediate(imm as i16),
                }
            }
            // Immediate to register
            0b1011_0000..=0b1011_1111 => {
                // Parse the bit fields
                let wide = (input[0] >> 4) & 1;
                let reg = input[0] & 0b111;
                let reg = Register::from_reg_w(Reg(reg), Wide(wide));

                let mut imm = input[1] as u16;
                let mut size = 2;
                if wide > 0 {
                    imm |= (input[2] as u16) << 8;
                    size += 1
                }

                // Update the input bytes
                input = &input[size..];

                // Add the decoded instruction to the list
                Instruction::Mov {
                    dest: Operand::Register(reg),
                    src: Operand::Immediate(imm as i16),
                }
            }
            // Memory to accumulator
            0b1010_0000..=0b1010_0001 => {
                // Parse the bit fields
                let wide = input[0] & 1;
                let mut imm = input[1] as u16;
                let mut size = 2;
                if wide > 0 {
                    imm |= (input[2] as u16) << 8;
                    size += 1
                }

                // Update the input bytes
                input = &input[size..];

                // Add the decoded instruction to the list
                Instruction::Mov {
                    dest: Operand::Register(Register::Ax),
                    src: Operand::Memory(Memory::direct_address(imm)),
                }
            }
            // Accumulator to Memory
            0b1010_0010..=0b1010_0011 => {
                // Parse the bit fields
                let wide = input[0] & 1;
                let mut imm = input[1] as u16;
                let mut size = 2;
                if wide > 0 {
                    imm |= (input[2] as u16) << 8;
                    size += 1
                }

                // Update the input bytes
                input = &input[size..];

                // Add the decoded instruction to the list
                Instruction::Mov {
                    dest: Operand::Memory(Memory::direct_address(imm)),
                    src: Operand::Register(Register::Ax),
                }
            }
            _ => return Err(DecoderError::UnknownInstruction(input[0]).into()),
        };

        eprintln!("TEST: {instr:x?}");
        eprintln!("ASM:  {instr}");

        // Add the instruction to the instruction stream
        res.push(instr);
    }

    Ok(res)
}

/// Parse the mov instruction
pub fn parse_mov_instr(
    input: &[u8],
    mod_: Mod,
    reg: Reg,
    wide: Wide,
    rm: Rm,
) -> Result<(Operand, Operand, usize)> {
    let res = match mod_.0 {
        0b00 => {
            let reg = Register::from_reg_w(reg, wide);

            // Special case the RM 0b110 case as a direct address
            let (mem, size) = if rm.0 == 0b110 {
                let address = input[2] as u16 | (input[3] as u16) << 8;
                (Memory::direct_address(address), 4)
            } else {
                (Memory::from_mod_rm(mod_, rm, wide)?, 2)
            };

            (Operand::Register(reg), Operand::Memory(mem), size)
        }
        0b01 => {
            let reg = Register::from_reg_w(reg, wide);
            let displacement = input[2] as i8 as i16;

            let mem = Memory::from_mod_rm(mod_, rm, wide)?.with_displacement(displacement);

            (Operand::Register(reg), Operand::Memory(mem), 3)
        }
        0b10 => {
            let reg = Register::from_reg_w(reg, wide);
            let displacement = input[2] as i16 | (input[3] as i16) << 8;

            let mem = Memory::from_mod_rm(mod_, rm, wide)?.with_displacement(displacement);

            (Operand::Register(reg), Operand::Memory(mem), 4)
        }
        0b11 => {
            let reg = Register::from_reg_w(reg, wide);
            let rm_reg = Register::from_reg_w(Reg(rm.0), wide);

            (Operand::Register(reg), Operand::Register(rm_reg), 2)
        }
        _ => {
            // Can't be reached due to the bitwise and 0b11
            unsafe {
                std::hint::unreachable_unchecked();
            }
        }
    };

    Ok(res)
}
