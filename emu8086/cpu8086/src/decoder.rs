use crate::instruction::{Instruction, Mod, Operand, Reg, Rm, Wide, Repeat};
use crate::memory_operand::MemoryOperand;
use crate::register::{Register, SegmentRegister};
use crate::memory::{Memory, Address};
use crate::emu::RegisterState;
use crate::const_checks::{is_valid_address_size, If, True};

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Attempted to parse an unknown instruction at offset {1:x?}: {0:#x}")]
    UnknownInstruction(u8, Address),

    #[error("Attempted to parse an unknown repeat opcode at offset {1:x?}: {0:#x}")]
    UnknownRepeatOpcode(u8, Address),
}

/// Decode the given byte stream into a set of [`Instruction`]
#[allow(
    clippy::too_many_lines, 
    clippy::similar_names, 
    clippy::cast_possible_wrap, 
    clippy::unusual_byte_groupings,
    clippy::verbose_bit_mask
)]
pub fn decode_instruction<const SIZE: usize>(cpu: &mut RegisterState, memory: &Memory<SIZE>)  
    -> Result<Instruction> 
where If<{ is_valid_address_size(SIZE) }>: True {
    let address = Address(cpu.ip as usize);

    macro_rules! unknown_instr {
        () => {{
            println!("{}:{}", file!(), line!());
            return Err(Error::UnknownInstruction(memory.read::<u8>(address)?, address)
            .into())
        }}
    }

    let mut segment = None;

    let mut input = &memory.memory[cpu.ip as usize..];
    let res;

    loop {
        /// Insert a jump/loop instruction with its label into the instruction stream
        macro_rules! jump_instr {
            ($jmp:ident) => {{
                // +2 since the instruction is 2 bytes
                let instr = Instruction::$jmp { offset: input[1] as i8 + 2 };
                let size = 2;
                (instr, size)
            }}
        }

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
                let (reg, rm, mut size) = parse_mod_reg_rm_instr(input, Wide(w), segment.take())?;
                
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
                    0b001110_00 => Instruction::Cmp { left: dest, right: src }, 
                    0b001000_00 => Instruction::And { dest, src }, 
                    0b001100_00 => Instruction::Xor { dest, src }, 

                    // ERROR IN MANUAL: This is the correct decoding for Test
                    0b100001_00 => Instruction::Test { dest, src }, 
                    0b111101_00 => {
                        let opcode = input[1] >> 3 & 0b111;

                        match opcode {
                            0b000 => {
                                let mut imm = u16::from(input[size]);
                                size += 1;

                                if w > 0 {
                                    imm |= (u16::from(input[size])) << 8;
                                    size += 1;
                                }

                                let src = Operand::Immediate(imm as i16);
                                Instruction::Test { dest: rm, src }
                            }
                            0b001 => unknown_instr!(),
                            0b010 => Instruction::Not { src },
                            0b011 => Instruction::Neg { src },
                            0b100 => Instruction::Mul { src },
                            0b101 => Instruction::Imul { src },
                            0b110 => Instruction::Div { src },
                            0b111 => Instruction::Idiv { src },
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
                            0b000 => Instruction::Rol { src: rm, count },
                            0b001 => Instruction::Ror { src: rm, count },
                            0b010 => Instruction::Rcl { src: rm, count },
                            0b011 => Instruction::Rcr { src: rm, count },
                            0b100 => Instruction::Shl { src: rm, count },
                            0b101 => Instruction::Shr { src: rm, count },
                            0b111 => Instruction::Sar { src: rm, count },
                            _ => unknown_instr!()
                        }
                    }
                    0b111111_00 => {
                        if input[1] >> 3 & 0b111 == 0b000 {
                            Instruction::Inc { src }
                        } else if input[1] >> 3 & 0b111 == 0b001 {
                            Instruction::Dec { src }
                        } else {
                            unknown_instr!()
                        }
                    }
                    _ => unknown_instr!()
                };

                (instr, size)
            }
            // <OPERATION> Immediate to register/memory
            0b1000_0000..=0b1000_0011 // sbb/cmp/and/add/adc/or
            | 0b1100_0110..=0b1100_0111 // mov
            => {
                // Parse the bit fields
                let wide = input[0] & 1;
                let s = (input[0] >> 1) & 1;

                // Parse the mod/reg/rm second byte
                let (_, rm, mut size) = parse_mod_reg_rm_instr(input, Wide(wide), segment.take())?;

                // Continue parsing the immediate after the parsed size is there was
                // a displacement or not
                let mut imm = input[size] as u8;
                size += 1;

                // Handle the s/d bit for the various opcodes
                let imm= match input[0] {
                    0b1100_0110 => {
                        // Do not get a u16 immediate for a byte mov
                        // imm as u16
                        0xdead

                    }
                    0b1100_0111 => {
                        // If we definitly have a wide move, get a u16 immediate
                        let res = imm as u16 | u16::from(input[size]) << 8;
                        size += 1;
                        res
                    }
                    _ if wide > 0 && s == 0 => {
                        // If wide bit is set, two cases arise:
                        // * Sign bit is it - the u8 becomes a u16 so the wide bit is satisfied
                        // * Sign not set - need a u16 immediate, so we need to read another byte
                        let res = imm as u16 | (u16::from(input[size]) << 8);
                        size += 1;
                        res
                    }
                    _ => {
                        // imm as u16
                        0xdead
                        // Do nothing here
                    }
                };

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
                    0b111 =>  Instruction::Cmp { left: dest, right: src },
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
            | 0b1010_1000..=0b1010_1001 // test
            => {
                // Parse the bit fields
                let wide = input[0] & 1;

                let mut size = 1;

                let mut imm = u16::from(input[size]);
                let mut accumulator = Register::Al;
                size += 1;
                if wide > 0 {
                    imm |= u16::from(input[size]) << 8;
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
                    0b0011_1100 => Instruction::Cmp { left: dest, right: src },
                    0b0010_0100 => Instruction::And { dest, src },
                    0b1010_1000 => Instruction::Test { dest, src },
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

                let mut imm = u16::from(input[1]);
                let mut size = 2;
                if wide > 0 {
                    imm |= u16::from(input[2]) << 8;
                    size += 1;
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
                let mut imm = u16::from(input[1]);
                let mut size = 2;
                if wide > 0 {
                    imm |= u16::from(input[2]) << 8;
                    size += 1;
                }

                // Add the decoded instruction to the list
                let instr = Instruction::Mov {
                    dest: Operand::Register(Register::Ax),
                    src: Operand::Memory(MemoryOperand::direct_address(imm, Wide(wide))
                        .with_segment(segment.take())),
                };

                (instr, size)
            }
            // MOV Accumulator to Memory
            0b1010_0010..=0b1010_0011 => {
                // Parse the bit fields
                let wide = input[0] & 1;
                let mut imm = u16::from(input[1]);
                let mut size = 2;
                if wide > 0 {
                    imm |= u16::from(input[2]) << 8;
                    size += 1;
                }

                // Add the decoded instruction to the list
                let instr = Instruction::Mov {
                    dest: Operand::Memory(MemoryOperand::direct_address(imm, Wide(wide)).with_segment(segment)),
                    src: Operand::Register(Register::Ax),
                };

                (instr, size)
            }
            // MOV Register/memory to segment register
            0b1000_1110 => {
                let (segment_reg, operand, size) = parse_mod_segreg_rm_instr(input, segment.take())?;

                let instr = Instruction::Mov {
                    dest: Operand::SegmentRegister(segment_reg),
                    src: operand,
                };

                (instr, size)
            }
            // MOV Register/memory to segment register
            0b1000_1100 => {
                let (segment_reg, operand, size) = parse_mod_segreg_rm_instr(input, segment.take())?;

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
                let (_reg, rm, size) = parse_mod_reg_rm_instr(input, Wide(1), segment.take())?;

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
                let segment_reg: SegmentRegister = match (input[0] >> 3) & 0b11 {
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
                let (reg, rm, size) = parse_mod_reg_rm_instr(input, wide, segment.take())?;

                let instr = Instruction::Xchg {
                    left: reg,
                    right: rm,
                };

                (instr, size)
            }
            // XCHG with accumulator
            0b1001_0001..=0b1001_0111 => {
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
                    src: Operand::Immediate(i16::from(data)),

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
                    dest: Operand::Immediate(i16::from(port)),
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
                let (reg, mut rm, size) = parse_mod_reg_rm_instr(input, Wide(1), segment.take())?;

                // Remove the memory size for a Memory src
                if let Operand::Memory(ref mut memory) = &mut rm {
                    memory.size = None;
                }

                let instr = Instruction::Lea { dest: reg, src: rm };
                (instr, size)
            }
            0b1100_0101 => {
                let (reg, mut rm, size) = parse_mod_reg_rm_instr(input, Wide(1), segment.take())?;

                // Remove the memory size for a Memory src
                if let Operand::Memory(ref mut memory) = &mut rm {
                    memory.size = None;
                }

                let instr = Instruction::Lds { dest: reg, src: rm };

                (instr, size)
            }
            0b1100_0100 => {
                let (reg, mut rm, size) = parse_mod_reg_rm_instr(input, Wide(1), segment.take())?;

                // Remove the memory size for a Memory src
                if let Operand::Memory(memory) = &mut rm {
                    memory.size = None;
                }

                let instr = Instruction::Les { dest: reg, src: rm };
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
            // INC REG
            0b0100_0000..=0b0100_0111 => {
                let reg = Register::from_reg_w(Reg(input[0] & 0b111), Wide(1));

                let instr = Instruction::Inc {
                    src: Operand::Register(reg),
                };

                let size = 1;

                (instr, size)

            }
            // INC DEC
            0b0100_1000..=0b0100_1111 => {
                let reg = Register::from_reg_w(Reg(input[0] & 0b111), Wide(1));

                let instr = Instruction::Dec  {
                    src: Operand::Register(reg),
                };

                let size = 1;

                (instr, size)
            }
            0b1111_0010 | 0b1111_0011 => {
                let repeat = match input[0] & 1  {
                    0 => Some(Repeat::WhileClearZeroFlag),
                    1 => Some(Repeat::WhileSetZeroFlag),

                    // SAFETY: Unreachable due to the bitand above
                    _ => unsafe { std::hint::unreachable_unchecked() }
                };

                let instr = match input[1] {
                    0b1010_0100 => Instruction::MoveByte { repeat },
                    0b1010_0101 => Instruction::MoveWord { repeat },
                    0b1010_0110 => Instruction::CmpByte { repeat },
                    0b1010_0111 => Instruction::CmpWord { repeat },
                    0b1010_1110 => Instruction::ScanByte { repeat },
                    0b1010_1111 => Instruction::ScanWord { repeat },
                    0b1010_1100 => Instruction::LoadByte { repeat },
                    0b1010_1101 => Instruction::LoadWord { repeat },
                    0b1010_1010 => Instruction::StoreByte { repeat },
                    0b1010_1011 => Instruction::StoreWord { repeat },
                    _ => return Err(Error::UnknownRepeatOpcode(input[1], address).into())
                };

                (instr, 2)
            }
            0b1010_0100 => {
                let size = 1;
                let instr = Instruction::MoveByte { repeat: None };
                (instr, size)
            }
            0b1010_0101 => {
                let instr = Instruction::MoveWord { repeat: None };
                let size = 1;
                (instr, size)
            }
            0b1010_0110 => {
                let instr = Instruction::CmpByte { repeat: None };
                let size = 1;
                (instr, size)
            }
            0b1010_0111 => {
                let instr = Instruction::CmpWord { repeat: None };
                let size = 1;
                (instr, size)
            }
            0b1010_1110 => {
                let instr = Instruction::ScanByte { repeat: None };
                let size = 1;
                (instr, size)
            }
            0b1010_1111 => {
                let instr = Instruction::ScanWord { repeat: None };
                let size = 1;
                (instr, size)
            }
            0b1010_1100 => {
                let instr = Instruction::LoadByte { repeat: None };
                let size = 1;
                (instr, size)
            }
            0b1010_1101 => {
                let instr = Instruction::LoadWord { repeat: None };
                let size = 1;
                (instr, size)
            }
            0b1010_1010 => {
                let instr = Instruction::StoreByte { repeat: None };
                let size = 1;
                (instr, size)
            }
            0b1010_1011 => {
                let instr = Instruction::StoreWord { repeat: None };
                let size = 1;
                (instr, size)
            }
            0b1100_0010 => {
                let offset = i16::from_le_bytes(input[1..3].try_into().unwrap());
                let instr = Instruction::ReturnWithOffset { offset };
                let size = 3;
                (instr, size)
            }
            0b1100_0011 => {
                let instr = Instruction::Return;
                let size = 1;
                (instr, size)
            }
            0b0111_0100 => jump_instr!(JumpEqual),
            0b0111_0101 => jump_instr!(JumpNotEqual),
            0b0111_1100 => jump_instr!(JumpLessThan),
            0b0111_1110 => jump_instr!(JumpLessThanEqual),
            0b0111_0010 => jump_instr!(JumpBelow),
            0b0111_0110 => jump_instr!(JumpBelowEqual),
            0b0111_1010 => jump_instr!(JumpParityEven),
            0b0111_0000 => jump_instr!(JumpOverflow),
            0b0111_1000 => jump_instr!(JumpSign),
            0b0111_1101 => jump_instr!(JumpNotLessThan),
            0b0111_1111 => jump_instr!(JumpNotLessThanEqual),
            0b0111_0011 => jump_instr!(JumpNotBelow),
            0b0111_0111 => jump_instr!(JumpNotBelowEqual),
            0b0111_1011 => jump_instr!(JumpParityOdd),
            0b0111_0001 => jump_instr!(JumpNotOverflow),
            0b0111_1001 => jump_instr!(JumpNotSign),
            0b1110_0010 => jump_instr!(Loop),
            0b1110_0001 => jump_instr!(LoopWhileZero),
            0b1110_0000 => jump_instr!(LoopWhileNotZero),
            0b1110_0011 => jump_instr!(JumpCxZero),
            0b1100_1101 => {
                let instr = Instruction::Interrupt { vector: input[1] };
                let size = 2;
                (instr, size)
            }
            0b1100_1100 => {
                let instr = Instruction::Interrupt { vector: 3 };
                let size = 1;
                (instr, size)
            }
            0b1100_1110 => {
                let instr = Instruction::InterruptOnOverflow;
                let size = 1;
                (instr, size)
            }
            0b1100_1111 => {
                let instr = Instruction::InterruptReturn;
                let size = 1;
                (instr, size)
            }
            0b1111_1000 => {
                let instr = Instruction::ClearCarry;
                let size = 1;
                (instr, size)
            }
            0b1111_0101 => {
                let instr = Instruction::ComplementCarry;
                let size = 1;
                (instr, size)
            }
            0b1111_1001 => {
                let instr = Instruction::SetCarry;
                let size = 1;
                (instr, size)
            }
            0b1111_1100 => {
                let instr = Instruction::ClearDirection;
                let size = 1;
                (instr, size)
            }
            0b1111_1101 => {
                let instr = Instruction::SetDirection;
                let size = 1;
                (instr, size)
            }
            0b1111_1010 => {
                let instr = Instruction::ClearInterrupt;
                let size = 1;
                (instr, size)
            }
            0b1111_1011 => {
                let instr = Instruction::SetInterrupt;
                let size = 1;
                (instr, size)
            }
            0b1111_0100 => {
                let instr = Instruction::Halt;
                let size = 1;
                (instr, size)
            }
            0b1001_1011 => {
                let instr = Instruction::Wait;
                let size = 1;
                (instr, size)
            }
            0b1111_0000 => {
                let instr = Instruction::Lock;
                let size = 1;
                (instr, size)
            }
            0b001_00_110 | 0b001_01_110  | 0b001_10_110  | 0b001_11_110 => {
                // Parse and convert the bits into a SegmentRegister
                let segment_reg: SegmentRegister = match (input[0] >> 3) & 0b11 {
                    0b00 => SegmentRegister::Es,
                    0b01 => SegmentRegister::Cs,
                    0b10 => SegmentRegister::Ss,
                    0b11 => SegmentRegister::Ds,
                    // SAFETY: Cannot be reached due to the bitand of 0b11 above
                    _ => unsafe { std::hint::unreachable_unchecked() },
                };

                // Manually update the current segment 
                segment = Some(segment_reg);

                // Continue the instruction stream without inserting a new instruction
                input = &input[1..];

                // Increment the IP from the CPU for reading this segment byte
                cpu.ip += 1;

                // Continue to parse the next instruction now that we've read this prefix
                continue;
            }
               
            0b1111_1111 => {
                let (_reg, rm, size) = parse_mod_reg_rm_instr(input, Wide(1), segment.take())?;

                let opcode = input[1] >> 3 & 0b111;

                let instr = match opcode {
                    // INC mem16
                    0b000 => Instruction::Inc { src: rm },
                    // DEC mem16
                    0b001 => Instruction::Dec{ src: rm },
                    // CALL mem16
                    0b010 => Instruction::Call { dest: rm },
                    // JUMP mem16
                    0b100 => Instruction::Jump { dest: rm },
                    // PUSH mem16
                    0b110 => Instruction::Push { src: rm },
                    _ =>  unknown_instr!()
                };

                (instr, size)
            }
            _ => unknown_instr!()
        };

        // eprintln!("TEST: {instr:x?}");
        // eprintln!("ASM:  {instr}");

        // Update the input bytes
        cpu.ip += u16::try_from(size).unwrap();

        // Add the instruction to the instruction stream
        res = Some(instr);

        break
    }

    Ok(res.unwrap())
}

/// Parse an instruction with the "mod|reg|r/m" bit pattern
pub fn parse_mod_reg_rm_instr(input: &[u8], wide: Wide, segment: Option<SegmentRegister>) -> Result<(Operand, Operand, usize)> {
    let rm   = Rm(input[1] & 0b111);
    let reg  = Reg((input[1] >> 3) & 0b111);
    let mod_ = Mod((input[1] >> 6) & 0b11);

    let result = match mod_.0 {
        0b00 => {
            let reg = Register::from_reg_w(reg, wide);

            // Special case the RM 0b110 case as a direct address
            let (mem, size) = if rm.0 == 0b110 {
                let address = u16::from(input[2]) | u16::from(input[3]) << 8;
                (MemoryOperand::direct_address(address, wide).with_segment(segment), 4)
            } else {
                (MemoryOperand::from_mod_rm(mod_, rm, wide)?.with_segment(segment), 2)
            };

            (Operand::Register(reg), Operand::Memory(mem), size)
        }
        0b01 => {
            let reg = Register::from_reg_w(reg, wide);

            #[allow(clippy::cast_possible_wrap)]
            let displacement = i16::from(input[2] as i8);

            let mem = MemoryOperand::from_mod_rm(mod_, rm, wide)?
                .with_displacement(displacement)
                .with_segment(segment);

            (Operand::Register(reg), Operand::Memory(mem), 3)
        }
        0b10 => {
            let reg = Register::from_reg_w(reg, wide);
            let displacement = i16::from(input[2]) | i16::from(input[3]) << 8;

            let mem = MemoryOperand::from_mod_rm(mod_, rm, wide)?
                .with_displacement(displacement)
                .with_segment(segment);

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

    Ok(result)
}

/// Parse an instruction with the "mod|segreg|r/m" bit pattern
pub fn parse_mod_segreg_rm_instr(input: &[u8], segment: Option<SegmentRegister>) -> Result<(SegmentRegister, Operand, usize)> {
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
                let address = u16::from(input[2]) | u16::from(input[3]) << 8;
                (MemoryOperand::direct_address(address, wide).with_segment(segment), 4)
            } else {
                (MemoryOperand::from_mod_rm(mod_, Rm(rm), wide)?.with_segment(segment), 2)
            };

            (segment_reg, Operand::Memory(mem), size)
        }
        0b01 => {
            #[allow(clippy::cast_possible_wrap)]
            let displacement = i16::from(input[2] as i8);
            let mem = MemoryOperand::from_mod_rm(mod_, Rm(rm), wide)?
                .with_displacement(displacement)
                .with_segment(segment);

            (segment_reg, Operand::Memory(mem), 3)
        }
        0b10 => {
            let displacement = i16::from(input[2]) | i16::from(input[3]) << 8;
            let mem = MemoryOperand::from_mod_rm(mod_, Rm(rm), wide)?
                .with_displacement(displacement)
                .with_segment(segment);

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
