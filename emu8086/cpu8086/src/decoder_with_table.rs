use crate::const_checks::{is_valid_address_size, If, True};
use crate::emu::RegisterState;
use crate::instruction::{Instruction, Mod, Operand, Reg, Rm, Wide};
use crate::memory::{Address, Memory};
use crate::memory_operand::MemoryOperand;
use crate::register::{Register, SegmentRegister};

use anyhow::Result;
use thiserror::Error;

/// Decode the given byte stream into a set of [`Instruction`]
pub fn decode_instruction<const SIZE: usize>(
    cpu: &mut RegisterState,
    memory: &Memory<SIZE>,
) -> Result<Instruction>
where
    If<{ is_valid_address_size(SIZE) }>: True,
{
    crate::decoder_with_match::decode_instruction(cpu, memory)
}

/*
#[derive(Error, Debug)]
enum DecodeTableError {
    #[error("Wide bit not found during decode")]
    WideBitNotFound,

    #[error("Reg bit not found during decode")]
    RegBitNotFound,

    #[error("Rm bit not found during decode")]
    RmBitNotFound,

    #[error("Mod bit not found during decode")]
    ModBitNotFound,

    #[error("D bit not found during decode")]
    DBitNotFound,
}

/// Decode the given byte stream into a set of [`Instruction`]
pub fn decode_instruction_with_table<const SIZE: usize>(
    cpu: &mut RegisterState,
    memory: &Memory<SIZE>,
) -> Result<Instruction>
where
    If<{ is_valid_address_size(SIZE) }>: True,
{
    'next_instr_encoding: for (
        index,
        InstructionEncoding {
            opcode,
            bit_encodings,
        },
    ) in INSTRUCTION_TABLE.iter().enumerate()
    {
        let mut bits = [None; std::mem::variant_count::<BitPurpose>()];

        let mut curr_bits = 0;
        let mut bits_left = 0;
        let mut offset = 0_u16;
        let mut addr = Address(cpu.ip as usize);

        println!("INDEX {index} {bit_encodings:x?}");

        for encoding in bit_encodings {
            // Break from the loop once encoding is not a Some(BitsEncoding {})
            let Some(BitsEncoding { encoding, count, shift, value }) = encoding else {
                break;
            };

            // No need to read the next byte if it is not a wide instruction
            if matches!(encoding, BitPurpose::DataIfW)
                && matches!(bits[BitPurpose::W as usize], Some(0))
            {
                continue;
            }

            // If we don't have any bits left to test, read more bits from the IP
            if bits_left == 0 && *count > 0 {
                // Read and reverse the bits in the next byte of the instruction stream
                curr_bits = memory.read::<u8>(addr.offset(offset as usize))?;
                println!(
                    "{:x?} Read byte: {curr_bits:08b} {curr_bits:#x}",
                    addr.offset(offset as usize)
                );

                bits_left = 8;

                // Increment the read address to the next byte for the next read
                offset += 1;
            }

            // Reduce the number of bits left by this literals bit count
            bits_left -= count;

            // Get the bit mask for this encoding
            let mut bit_mask = 0xff;
            if *count < 8 {
                bit_mask = !(bit_mask << count);
            }

            match value {
                Some(value) => {
                    // Current bits: 0b1000_1011 Literal value: 0b100010 Count: 6
                    // Bits left:    8 - 6 = 2
                    // Checked bits: 0b1000_1011
                    // Checked bits >> 2: 0b0010_0010
                    // Checked bits &= 6: 0b0010_0010
                    if *count > 0 {
                        let mut checked_bits = curr_bits;
                        checked_bits >>= bits_left;
                        checked_bits &= bit_mask;

                        // If the next bits in the bit stream aren't the required value bits,
                        // go to the next instruction encoding
                        if checked_bits != *value {
                            continue 'next_instr_encoding;
                        }
                    }

                    // Found the correct bits. Set them in the found bits list.
                    bits[*encoding as usize] = Some(*value);
                }
                None => {
                    let mut value = curr_bits;
                    value >>= bits_left;
                    value &= bit_mask;
                    bits[*encoding as usize] = Some(value);
                }
            }

            // Set the address bytes when we encounter the RM bits
            if matches!(encoding, BitPurpose::Rm) {
                match bits[BitPurpose::Mod as usize] {
                    Some(1) => {
                        let byte = memory.read::<u8>(addr.offset(offset as usize))?;
                        offset += 1;
                        bits[BitPurpose::AddressLow as usize] = Some(byte);
                    }
                    Some(2) => {
                        let byte = memory.read::<u8>(addr.offset(offset as usize))?;
                        offset += 1;
                        bits[BitPurpose::AddressLow as usize] = Some(byte);

                        let byte = memory.read::<u8>(addr.offset(offset as usize))?;
                        offset += 1;
                        bits[BitPurpose::AddressHigh as usize] = Some(byte);
                    }
                    // Special case for direct address
                    Some(0) => {
                        if matches!(bits[BitPurpose::Rm as usize], Some(0b110)) {
                            let byte = memory.read::<u8>(addr.offset(offset as usize))?;
                            offset += 1;
                            bits[BitPurpose::AddressLow as usize] = Some(byte);

                            let byte = memory.read::<u8>(addr.offset(offset as usize))?;
                            offset += 1;
                            bits[BitPurpose::AddressHigh as usize] = Some(byte);
                        }
                    }
                    Some(3 | 0xff) => {
                        // No need to read more memory for values 3 or the fake 0xff
                    }
                    None => {
                        panic!("Table index {index} | Encountered RM bits before MOD was set");
                    }
                    mod_ => {
                        dbg!(mod_);
                        unreachable!()
                    }
                }
            }
        }

        // If we reach this point, we have a valid encoding
        println!("INDEX: {index} Bits: {bits:x?}");

        let (op1, op2) = get_operands_from_bits(bits)?;

        println!("OP1 {op1:x?}");
        println!("OP2 {op2:x?}");

        let result = get_instruction(opcode, op1, op2);

        println!("{:#x}: {result}", cpu.ip);

        // Increment the ip
        cpu.ip += offset;

        // Return the decoded instruction
        return Ok(result);
    }

    println!("Size: {}", std::mem::size_of_val(INSTRUCTION_TABLE));
    panic!(
        "Instruction not decoded at {:#x}: {:b} {:b}",
        cpu.ip,
        memory.read::<u8>(Address(cpu.ip as usize))?,
        memory.read::<u8>(Address(cpu.ip as usize + 1))?,
    );
}

pub fn get_operands_from_bits(
    bits: [Option<u8>; std::mem::variant_count::<BitPurpose>()],
) -> Result<(Option<Operand>, Option<Operand>)> {
    let mod_ = Mod(bits[BitPurpose::Mod as usize].ok_or_else(|| DecodeTableError::ModBitNotFound)?);
    let rm = Rm(bits[BitPurpose::Rm as usize].ok_or_else(|| DecodeTableError::RmBitNotFound)?);
    let wide = Wide(bits[BitPurpose::W as usize].ok_or_else(|| DecodeTableError::WideBitNotFound)?);
    let d = bits[BitPurpose::D as usize].ok_or_else(|| DecodeTableError::DBitNotFound)? > 0;
    let data = bits[BitPurpose::Data as usize];
    let data2 = bits[BitPurpose::DataIfW as usize];
    let addr_lo = bits[BitPurpose::AddressLow as usize];
    let addr_hi = bits[BitPurpose::AddressHigh as usize];

    let segment = match bits[BitPurpose::Sr as usize] {
        Some(0b00) => Some(SegmentRegister::Es),
        Some(0b01) => Some(SegmentRegister::Cs),
        Some(0b10) => Some(SegmentRegister::Ss),
        Some(0b11) => Some(SegmentRegister::Ds),
        Some(_) => unreachable!(),
        None => None,
    };
    dbg!(mod_, rm, wide, d, data, data2);

    macro_rules! reg_or_imm {
        () => {
            if let Some(reg) = bits[BitPurpose::Reg as usize] {
                let reg = Reg(bits[BitPurpose::Reg as usize]
                    .ok_or_else(|| DecodeTableError::RegBitNotFound)?);

                Some(Operand::Register(Register::from_reg_w(reg, wide)))
            } else if data.is_some() {
                let mut imm = data.unwrap() as u16;
                if wide.0 == 1 {
                    imm |= (data2.unwrap() as u16) << 8
                }

                Some(Operand::Immediate(imm as i16))
            } else if let Some(seg) = segment {
                Some(Operand::SegmentRegister(seg))
            } else {
                None
            }
        };
    }

    let (op1, op2) =
        match mod_.0 {
            0b00 => {
                let first_op = reg_or_imm!();

                // Special case the RM 0b110 case as a direct address
                let mem = if rm.0 == 0b110 {
                    let address = addr_lo.unwrap() as u16 | (addr_hi.unwrap() as u16) << 8;
                    MemoryOperand::direct_address(address, wide).with_segment(segment)
                } else {
                    MemoryOperand::from_mod_rm(mod_, rm, wide)?.with_segment(segment)
                };

                (first_op, Operand::Memory(mem))
            }
            0b01 => {
                let first_op = reg_or_imm!();

                let displacement = addr_lo.unwrap() as i8 as i16;

                let mem = MemoryOperand::from_mod_rm(mod_, rm, wide)?
                    .with_displacement(displacement)
                    .with_segment(segment);

                (first_op, Operand::Memory(mem))
            }
            0b10 => {
                let first_op = reg_or_imm!();

                let displacement = addr_lo.unwrap() as i16 | (addr_hi.unwrap() as i16) << 8;

                let mem = MemoryOperand::from_mod_rm(mod_, rm, wide)?
                    .with_displacement(displacement)
                    .with_segment(segment);

                (first_op, Operand::Memory(mem))
            }
            0b11 => {
                let reg = Reg(bits[BitPurpose::Reg as usize]
                    .ok_or_else(|| DecodeTableError::RegBitNotFound)?);
                let reg = Register::from_reg_w(reg, wide);
                let rm_reg = Register::from_reg_w(Reg(rm.0), wide);

                (Some(Operand::Register(reg)), Operand::Register(rm_reg))
            }
            0xff => {
                // Fake Mod value for immediates
                let reg = Reg(bits[BitPurpose::Reg as usize]
                    .ok_or_else(|| DecodeTableError::RegBitNotFound)?);

                let reg = Register::from_reg_w(reg, wide);
                let mut imm = u16::from(data.unwrap());
                if wide.0 > 0 {
                    imm |= (u16::from(data2.unwrap())) << 8;
                }

                (Some(Operand::Register(reg)), Operand::Immediate(imm as i16))
            }
            _ => {
                // Can't be reached due to the bitwise and 0b11
                unsafe {
                    std::hint::unreachable_unchecked();
                }
            }
        };

    Ok(if d {
        (op1, Some(op2))
    } else {
        (Some(op2), op1)
    })
}

fn get_instruction(opcode: &Opcode, op1: Option<Operand>, op2: Option<Operand>) -> Instruction {
    match opcode {
        Opcode::Mov => Instruction::Mov {
            dest: op1.unwrap(),
            src: op2.unwrap(),
        },
        Opcode::Push => Instruction::Push { src: op1.unwrap() },
        Opcode::Pop => Instruction::Pop { src: op1.unwrap() },
        Opcode::Xchg => {
            let mut res = Instruction::Xchg {
                left: op1.unwrap(),
                right: op2.unwrap(),
            };

            // xchg ax, ax is the nop instruction on 8086
            if matches!(
                res,
                Instruction::Xchg {
                    left: Operand::Register(Register::Ax),
                    right: Operand::Register(Register::Ax),
                }
            ) {
                res = Instruction::Nop;
            }

            res
        }
        Opcode::In => match op1 {
            Some(Operand::Register(reg)) => Instruction::In {
                dest: reg,
                src: op2.unwrap(),
            },
            _ => panic!("Invalid operand found for in"),
        },
        _ => panic!("Create opcode: {opcode:?}"),
    }
}
*/
