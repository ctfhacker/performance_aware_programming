use crate::instruction::{Instruction, Mod, Operand, Reg, Rm, Wide};
use crate::memory::Memory;
use crate::register::{Register, SegmentRegister};

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecoderError {
    #[error("Attempted to parse an unknown instruction at offset {1:#x}: {0:#x}")]
    UnknownInstruction(u8, usize),
}

#[derive(Debug, Copy, Clone)]
pub enum Repeat {
    /// Repeat/loop while zero flag is clear
    WhileClearZeroFlag,

    /// Repeat/loop while zero flag is set
    WhileSetZeroFlag
}

/// Decode a stream of bytes and return the decoded `Instruction`s
pub fn decode_stream(mut input: &[u8]) -> Result<Vec<Instruction>> {
    let mut res = Vec::new();

    let orig_input = input;

    macro_rules! unknown_instr {
        () => {{
            println!("{}:{}", file!(), line!());
            return Err(DecoderError::UnknownInstruction(
                input[0],
                input.as_ptr() as usize - orig_input.as_ptr() as usize,
            )
            .into())
        }}
    }

    // Currently set repeat prefix for string manipulation
    let mut repeat = None;

    while !input.is_empty() {
        eprintln!("{:#x} | OP: {:#x}", 
            input.as_ptr() as usize - orig_input.as_ptr() as usize,
            input[0]
        );

        let (instr, size) = match input[0] {
            // <OPERATION> Register/memory to/from register
            0b1000_1000..=0b1000_1011   // mov
            | 0b0000_0000..=0b0000_0011 // add
            | 0b0000_1000..=0b0000_1011 // or
            | 0b0001_0000..=0b0001_0011 // adc
            | 0b0001_1000..=0b0001_1011 // sbb
            | 0b0010_0000..=0b0010_0011 // and
            | 0b0010_1000..=0b0010_1011 // sub
            | 0b0011_1000..=0b0011_1011 // cmp
            | 0b1000_0100..=0b1000_0101 // test
            | 0b1111_1110               // inc/dec
            | 0b1111_0110..=0b1111_0111 // neg/mul/imul
            | 0b1101_0000..=0b1101_0011 // shl/sal/shr/sar/rol/ror/rcl/rcr
            | 0b0011_0000..=0b0011_0011 // xor
            => {
                // Parse the bit fields
                let w = input[0] & 1;
                let d = (input[0] >> 1) & 1 > 0;

                // Parse the mod/reg/rm second byte
                let (reg, rm, size) = parse_mod_reg_rm_instr(input, Wide(w))?;
                
                // Align the src/dest to the proper position based on the `d` flag
                let (dest, src) = if d {
                    (reg, rm)
                } else {
                    (rm, reg)
                };

                let instr = match input[0] & 0b01111_1100 {
                    0b100010_00 => Instruction::Mov { dest, src }, 
                    0b000000_00 => Instruction::Add { dest, src }, 
                    0b000010_00 => Instruction::Or  { dest, src }, 
                    0b000100_00 => Instruction::Adc { dest, src }, 
                    0b001010_00 => Instruction::Sub { dest, src }, 
                    0b000110_00 => Instruction::Sbb { dest, src }, 
                    0b001110_00 => Instruction::Cmp { dest, src }, 
                    0b001000_00 => Instruction::And { dest, src }, 
                    0b001100_00 => Instruction::Or  { dest, src }, 
                    0b100001_00 => Instruction::Test { dest, src }, 
                    0b111101_00 => {
                        let opcode = input[1] >> 3 & 0b111;

                        match opcode {
                            0b000 => Instruction::Test { dest, src },
                            0b001 => unknown_instr!(),
                            0b010  => Instruction::Not { src },
                            0b011 =>  Instruction::Neg { src },
                            0b100 =>  Instruction::Mul { dest, src },
                            0b101 =>  Instruction::Imul { dest, src },
                            0b110 =>  Instruction::Div { dest, src },
                            0b111 =>  Instruction::Idiv { dest, src },
                            // SAFETY: Bounded here by the bitand above
                            _ => unsafe { std::hint::unreachable_unchecked() }
                        }
                    }
                    0b110100_00 => {
                        // shl/sal/shr/sar/rol/ror/rcl/rcr
                        // v is the D flag for these opcodes
                        let v = d;

                        let count = if v {
                            Operand::Register(Register::Cl)
                        } else {
                            Operand::Immediate(1)
                        };

                        let opcode = input[1] >> 3 & 0b111;

                        match opcode {
                            0b000 => Instruction::Rol { src, count },
                            0b001 => Instruction::Ror { src, count },
                            0b010 => Instruction::Rcl { src, count },
                            0b011 => Instruction::Rcr { src, count },
                            0b100 => Instruction::Shl { src, count },
                            0b101 => Instruction::Shr { src, count },
                            0b111 => Instruction::Sar { src, count },
                            _ => unknown_instr!()
                        }
                    }
                    0b111111_00 => {
                        if input[1] >> 3 & 0b111 == 0b000 {
                            Instruction::Inc { dest }
                        } else if input[1] >> 3 & 0b111 == 0b001 {
                            Instruction::Dec { dest }
                        } else {
                            unknown_instr!()
                        }
                    }
                    _ => unknown_instr!()
                };

                (instr, size)
            }
            // <OPERATION> Immediate to register/memory
            0b0010_1000..=0b0010_1011   // sub
            | 0b1000_0000..=0b1000_0011 // sbb/cmp/and/add/adc/or
            | 0b1100_0110..=0b1100_0111 // mov
            | 0b0011_0100..=0b0011_0101 // mov
            | 0b1111_0110..=0b1111_0111 // test
            => {
                // Parse the bit fields
                let wide = input[0] & 1;
                let s = (input[0] >> 1) & 1;

                // Parse the mod/reg/rm second byte
                let (_, rm, mut size) = parse_mod_reg_rm_instr(input, Wide(wide))?;

                // Continue parsing the immediate after the parsed size is there was
                // a displacement or not
                let mut imm = input[size] as u16;
                size += 1;
                dbg!(format!("{:b}", input[0]), wide, s);

                // Handle the s/d bit for the various opcodes
                match input[0] {
                    0b1100_0110 => {
                        // Do not get a u16 immediate for a byte mov
                    }
                    0b1100_0111 => {
                        // If we definitly have a wide move, get a u16 immediate
                        imm |= (input[size] as u16) << 8;
                        size += 1;
                    }
                    x if wide > 0 && s == 0 => {
                        // If wide bit is set, two cases arise:
                        // * Sign bit is it - the u8 becomes a u16 so the wide bit is satisfied
                        // * Sign not set - need a u16 immediate, so we need to read another byte
                        imm |= (input[size] as u16) << 8;
                        size += 1;
                    }
                    _ => {
                        // Do nothing here
                    }
                }

                let dest = rm;
                let src = Operand::Immediate(imm as i16);

                let instr = match input[1] >> 3 & 0b111 {
                    // Both MOV and ADD have same middle three bits in the second byte
                    // Use the original opcode as the deciding factor
                    0b000 => if input[0] & 0b1111_1110 == 0b1100_0110 {
                        Instruction::Mov { dest, src }
                    } else if input[0] & 0b1111_1110 == 0b1111_0110 {
                        Instruction::Test { dest, src }
                    } else {
                        Instruction::Add { dest, src }
                    }
                    0b001 =>  Instruction::Or  { dest, src },
                    0b010 =>  Instruction::Adc { dest, src },
                    0b011 =>  Instruction::Sbb { dest, src },
                    0b100 =>  Instruction::And { dest, src },
                    0b101 =>  Instruction::Sub { dest, src },
                    0b110 =>  Instruction::Xor { dest, src },
                    0b111 =>  Instruction::Cmp { dest, src },
                    _ => unknown_instr!()
                };

                (instr, size)
            }
            // <OPERATION> Immediate to accumulator
            0b0000_0100..=0b0000_0101   // add
            | 0b0000_1100..=0b0000_1101 // or
            | 0b0001_0100..=0b0001_0101 // adc
            | 0b0010_1100..=0b0010_1101 // sub
            | 0b0010_0100..=0b0010_0101 // and
            | 0b0001_1100..=0b0001_1101 // sbb
            | 0b0011_0100..=0b0011_0101 // xor
            | 0b0011_1100..=0b0011_1101 // cmp
            => {
                // Parse the bit fields
                let wide = input[0] & 1;

                let mut size = 1;

                let mut imm = input[size] as u16;
                let mut accumulator = Register::Al;
                size += 1;
                if wide > 0 {
                    imm |= (input[size] as u16) << 8;
                    size += 1;
                    accumulator = Register::Ax;
                }

                let dest = Operand::Register(accumulator);
                let src = Operand::Immediate(imm as i16);

                let instr = match input[0] & 0b1111_1110 {
                    0b0000_0100 => Instruction::Add { dest, src },
                    0b0000_1100 => Instruction::Or  { dest, src },
                    0b0001_0100 => Instruction::Adc { dest, src },
                    0b0010_1100 => Instruction::Sub { dest, src },
                    0b0001_1100 => Instruction::Sbb { dest, src },
                    0b0011_0100 => Instruction::Xor { dest, src },
                    0b0011_1100 => Instruction::Cmp { dest, src },
                    0b0010_0100 => Instruction::And { dest, src },
                    _ => unknown_instr!()
                };

                (instr, size)
            }

            // MOV Immediate to register
            0b1011_0000..=0b1011_1111 => {
                // Parse the bit fields
                let wide = (input[0] >> 3) & 1;
                let reg = input[0] & 0b111;
                let reg = Register::from_reg_w(Reg(reg), Wide(wide));

                let mut imm = input[1] as u16;
                let mut size = 2;
                if wide > 0 {
                    imm |= (input[2] as u16) << 8;
                    size += 1
                }

                // Add the decoded instruction to the list
                let instr = Instruction::Mov {
                    dest: Operand::Register(reg),
                    src: Operand::Immediate(imm as i16),
                };

                (instr, size)
            }
            // MOV Memory to accumulator
            0b1010_0000..=0b1010_0001 => {
                // Parse the bit fields
                let wide = input[0] & 1;
                let mut imm = input[1] as u16;
                let mut size = 2;
                if wide > 0 {
                    imm |= (input[2] as u16) << 8;
                    size += 1
                }

                // Add the decoded instruction to the list
                let instr = Instruction::Mov {
                    dest: Operand::Register(Register::Ax),
                    src: Operand::Memory(Memory::direct_address(imm)),
                };

                (instr, size)
            }
            // MOV Accumulator to Memory
            0b1010_0010..=0b1010_0011 => {
                // Parse the bit fields
                let wide = input[0] & 1;
                let mut imm = input[1] as u16;
                let mut size = 2;
                if wide > 0 {
                    imm |= (input[2] as u16) << 8;
                    size += 1
                }

                // Add the decoded instruction to the list
                let instr = Instruction::Mov {
                    dest: Operand::Memory(Memory::direct_address(imm)),
                    src: Operand::Register(Register::Ax),
                };

                (instr, size)
            }
            // MOV Register/memory to segment register
            0b10001110 => {
                let (segment_reg, operand, size) = parse_mod_segreg_rm_instr(&input)?;

                let instr = Instruction::Mov {
                    dest: Operand::SegmentRegister(segment_reg),
                    src: operand,
                };

                (instr, size)
            }
            // MOV Register/memory to segment register
            0b1000_1100 => {
                let (segment_reg, operand, size) = parse_mod_segreg_rm_instr(&input)?;

                let instr = Instruction::Mov {
                    dest: operand,
                    src: Operand::SegmentRegister(segment_reg),
                };

                (instr, size)
            }

            // PUSH register
            0b0101_0000..=0b0101_0111 => {
                let reg = Register::from_reg_w(Reg(input[0] & 0b111), Wide(1));

                let size = 1;

                let instr = Instruction::Push {
                    src: Operand::Register(reg),
                };

                (instr, size)
            }
            // PUSH segment register
            0b000_00_110 | 0b000_01_110 | 0b000_10_110 | 0b000_11_110 => {
                // Parse and convert the bits into a SegmentRegister
                let segment_reg: SegmentRegister = match (input[1] >> 3) & 0b11 {
                    0b00 => SegmentRegister::Es,
                    0b01 => SegmentRegister::Cs,
                    0b10 => SegmentRegister::Ss,
                    0b11 => SegmentRegister::Ds,
                    // SAFETY: Cannot be reached due to the bitand of 0b11 above
                    _ => unsafe { std::hint::unreachable_unchecked() },
                };

                let size = 1;

                let instr = Instruction::Push {
                    src: Operand::SegmentRegister(segment_reg),
                };

                (instr, size)
            }
            // POP register/memory
            0b1000_1111 => {
                let (_reg, rm, size) = parse_mod_reg_rm_instr(input, Wide(1))?;

                let instr = Instruction::Pop { src: rm };

                (instr, size)
            }
            // POP register
            0b01011_000..=0b01011_111 => {
                let reg = Register::from_reg_w(Reg(input[0] & 0b111), Wide(1));

                let size = 1;

                let instr = Instruction::Pop {
                    src: Operand::Register(reg),
                };

                (instr, size)
            }
            // POP segment register
            0b000_00_111 | 0b000_01_111 | 0b000_10_111 | 0b000_11_111 => {
                // Parse and convert the bits into a SegmentRegister
                let segment_reg: SegmentRegister = match (input[1] >> 3) & 0b11 {
                    0b00 => SegmentRegister::Es,
                    0b01 => SegmentRegister::Cs,
                    0b10 => SegmentRegister::Ss,
                    0b11 => SegmentRegister::Ds,
                    // SAFETY: Cannot be reached due to the bitand of 0b11 above
                    _ => unsafe { std::hint::unreachable_unchecked() },
                };

                let size = 1;

                let instr = Instruction::Pop {
                    src: Operand::SegmentRegister(segment_reg),
                };

                (instr, size)
            }
            // XCHG register/memory with register
            0b1000_0110..=0b1000_0111 => {
                let wide = Wide(input[0] & 1);
                let (reg, rm, size) = parse_mod_reg_rm_instr(input, wide)?;

                let instr = Instruction::Xchg {
                    left: reg,
                    right: rm,
                };

                (instr, size)
            }
            // XCHG with accumulator
            0b1001_0000..=0b1001_0111 => {
                let reg = Register::from_reg_w(Reg(input[0] & 0b111), Wide(1));

                let instr = Instruction::Xchg {
                    left: Operand::Register(reg),
                    right: Operand::Register(Register::Ax),
                };

                let size = 1;

                (instr, size)
            }
            // NOP
            0b1001_0000 => {
                let instr = Instruction::Nop;
                let size = 1;
                (instr, size)
            }
            // IN (variable port)
            0b1110_0100..=0b1110_0101 => {
                let data = input[1];
                let dest = match input[0] & 1 {
                    0 => Register::Al,
                    1 => Register::Ax,
                    // SAFETY: Only 0 or 1 can be the value due to the bitand 1 above
                    _ => unsafe { std::hint::unreachable_unchecked() },
                };

                let size = 2;
                let instr = Instruction::In {
                    dest,
                    src: Operand::Immediate(data as i16),
                };
                (instr, size)
            }
            // IN (fixed port)
            0b1110_1100..=0b1110_1101 => {
                let dest = match input[0] & 1 {
                    0 => Register::Al,
                    1 => Register::Ax,
                    // SAFETY: Only 0 or 1 can be the value due to the bitand 1 above
                    _ => unsafe { std::hint::unreachable_unchecked() },
                };

                let size = 1;
                let instr = Instruction::In {
                    dest,
                    src: Operand::Register(Register::Dx),
                };
                (instr, size)
            }
            // OUT (variable port)
            0b1110_0110..=0b1110_0111 => {
                let port = input[1];
                let src = match input[0] & 1 {
                    0 => Register::Al,
                    1 => Register::Ax,
                    // SAFETY: Only 0 or 1 can be the value due to the bitand 1 above
                    _ => unsafe { std::hint::unreachable_unchecked() },
                };

                let size = 2;
                let instr = Instruction::Out {
                    dest: Operand::Immediate(port as i16),
                    src,
                };
                (instr, size)
            }
            // OUT (fixed port)
            0b1110_1110..=0b1110_1111 => {
                let src = match input[0] & 1 {
                    0 => Register::Al,
                    1 => Register::Ax,
                    // SAFETY: Only 0 or 1 can be the value due to the bitand 1 above
                    _ => unsafe { std::hint::unreachable_unchecked() },
                };

                let size = 1;
                let instr = Instruction::Out {
                    dest: Operand::Register(Register::Dx),
                    src,
                };
                (instr, size)
            }
            // XLAT (translate byte to AL)
            0b1101_0111 => {
                let size = 1;
                let instr = Instruction::Xlat;
                (instr, size)
            }
            0b1000_1101 => {
                let (reg, rm, size) = parse_mod_reg_rm_instr(input, Wide(1))?;
                let instr = Instruction::Lea { dest: reg, src: rm };
                (instr, size)
            }
            0b1100_0101 => {
                let (_reg, rm, size) = parse_mod_reg_rm_instr(input, Wide(1))?;
                let instr = Instruction::Lds { src: rm };
                (instr, size)
            }
            0b1100_0100 => {
                let (_reg, rm, size) = parse_mod_reg_rm_instr(input, Wide(1))?;
                let instr = Instruction::Les { src: rm };
                (instr, size)
            }
            0b1001_1111 => {
                let instr = Instruction::Lahf;
                let size = 1;
                (instr, size)
            }
            0b1001_1110 => {
                let instr = Instruction::Sahf;
                let size = 1;
                (instr, size)
            }
            0b1001_1100 => {
                let instr = Instruction::Pushf;
                let size = 1;
                (instr, size)
            }
            0b1001_1101 => {
                let instr = Instruction::Popf;
                let size = 1;
                (instr, size)
            }
            0b0011_0111 => {
                let instr = Instruction::Aaa;
                let size = 1;
                (instr, size)
            }
            0b0010_0111 => {
                let instr = Instruction::Daa;
                let size = 1;
                (instr, size)
            }
            0b0011_1111 => {
                let instr = Instruction::Aas;
                let size = 1;
                (instr, size)
            }
            0b0010_1111 => {
                let instr = Instruction::Das;
                let size = 1;
                (instr, size)
            }
            0b1101_0100 => {
                let instr = Instruction::Aam;
                let size = 2;
                (instr, size)
            }
            0b1101_0101 => {
                let instr = Instruction::Aad;
                let size = 2;
                (instr, size)
            }
            0b1001_1000 => {
                let instr = Instruction::Cbw;
                let size = 1;
                (instr, size)
            }
            0b1001_1001 => {
                let instr = Instruction::Cwd;
                let size = 1;
                (instr, size)
            }
            /// INC REG
            0b0100_0000..=0b0100_0111 => {
                let reg = Register::from_reg_w(Reg(input[0] & 0b111), Wide(1));

                let instr = Instruction::Inc {
                    dest: Operand::Register(reg),
                };

                let size = 1;

                (instr, size)

            }
            /// INC DEC
            0b0100_1000..=0b0100_1111 => {
                let reg = Register::from_reg_w(Reg(input[0] & 0b111), Wide(1));

                let instr = Instruction::Dec  {
                    dest: Operand::Register(reg),
                };

                let size = 1;

                (instr, size)
            }
            0b1111_0010 | 0b1111_0011 => {
                repeat = match input[0] & 1  {
                    0 => Some(Repeat::WhileClearZeroFlag),
                    1 => Some(Repeat::WhileSetZeroFlag),

                    // SAFETY: Unreachable due to the bitand above
                    _ => unsafe { std::hint::unreachable_unchecked() }
                };

                // Update the instruction stream 
                input = &input[1..];

                // Force continue to avoid adding a dummy instruction
                continue;
            }
            0b1111_1111 => {
                let (_reg, rm, size) = parse_mod_reg_rm_instr(input, Wide(1))?;

                let opcode = input[1] >> 3 & 0b111;

                let instr = match opcode {
                    // INC mem16
                    0b000 => Instruction::Inc { dest: rm },
                    // DEC mem16
                    0b001 => Instruction::Inc { dest: rm },
                    // PUSH mem16
                    0b110 => Instruction::Push { src: rm },
                    _ =>  unknown_instr!()
                };

                (instr, size)
            }
            _ => unknown_instr!()
        };

        // Update the input bytes
        input = &input[size..];

        eprintln!("TEST: {instr:x?}");
        eprintln!("ASM:  {instr}");

        // Add the instruction to the instruction stream
        res.push(instr);
    }

    Ok(res)
}

/// Parse an instruction with the "mod|reg|r/m" bit pattern
pub fn parse_mod_reg_rm_instr(input: &[u8], wide: Wide) -> Result<(Operand, Operand, usize)> {
    let rm = Rm((input[1] >> 0) & 0b111);
    let reg = Reg((input[1] >> 3) & 0b111);
    let mod_ = Mod((input[1] >> 6) & 0b11);

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

/// Parse an instruction with the "mod|segreg|r/m" bit pattern
pub fn parse_mod_segreg_rm_instr(input: &[u8]) -> Result<(SegmentRegister, Operand, usize)> {
    let rm = input[1] & 0b111;
    let mod_ = Mod((input[1] >> 6) & 0b11);

    // Parse and convert the bits into a SegmentRegister
    let segment_reg: SegmentRegister = match (input[1] >> 3) & 0b11 {
        0b00 => SegmentRegister::Es,
        0b01 => SegmentRegister::Cs,
        0b10 => SegmentRegister::Ss,
        0b11 => SegmentRegister::Ds,
        // SAFETY: Cannot be reached due to the bitand of 0b11 above
        _ => unsafe { std::hint::unreachable_unchecked() },
    };

    // Segment registers are always 16 bits
    let wide = Wide(1);

    let res = match mod_.0 {
        0b00 => {
            // Special case the RM 0b110 case as a direct address
            let (mem, size) = if rm == 0b110 {
                let address = input[2] as u16 | (input[3] as u16) << 8;
                (Memory::direct_address(address), 4)
            } else {
                (Memory::from_mod_rm(mod_, Rm(rm), wide)?, 2)
            };

            (segment_reg, Operand::Memory(mem), size)
        }
        0b01 => {
            let displacement = input[2] as i8 as i16;
            let mem = Memory::from_mod_rm(mod_, Rm(rm), wide)?.with_displacement(displacement);
            (segment_reg, Operand::Memory(mem), 3)
        }
        0b10 => {
            let displacement = input[2] as i16 | (input[3] as i16) << 8;
            let mem = Memory::from_mod_rm(mod_, Rm(rm), wide)?.with_displacement(displacement);
            (segment_reg, Operand::Memory(mem), 4)
        }
        0b11 => {
            let rm_reg = Register::from_reg_w(Reg(rm), wide);
            (segment_reg, Operand::Register(rm_reg), 2)
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
