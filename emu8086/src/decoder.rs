use crate::instruction::{Instruction, Operand};
use crate::register::Register;
use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecoderError {
    #[error("Attempted to parse an unknown instruction: {0:#x}")]
    UnknownInstruction(u8),
}

/// Decode a stream of bytes and return the decoded `Instruction`s
pub fn decode_stream(mut input: &[u8]) -> Result<Vec<Instruction>> {
    let mut res = Vec::new();

    while !input.is_empty() {
        match input[0] & 0b1111_1100 {
            0b1000_1000 => {
                //  76543210 76543210 76543210 76543210
                // +--------+--------+--------+--------+
                // |100010dw|modregrm|xxxxxxxx|xxxxxxxx|
                // +--------+--------+--------+--------+
                // Parse the bit fields for the MOV instruction
                let w = input[0] & 1;
                let d = (input[0] >> 1) & 1 > 0;
                let rm = (input[1] >> 0) & 0b111;
                let reg = (input[1] >> 3) & 0b111;
                let mod_ = (input[1] >> 6) & 0b11;

                assert!(mod_ == 0b11, "Mod value {mod_:b} unimplemented");

                let reg = Register::from_reg_w(reg, w);
                let rm_reg = Register::from_reg_w(rm, w);

                let instr = if d {
                    Instruction::Mov {
                        dest: Operand::Register(reg),
                        src: Operand::Register(rm_reg),
                    }
                } else {
                    Instruction::Mov {
                        dest: Operand::Register(rm_reg),
                        src: Operand::Register(reg),
                    }
                };

                // Add the instruction to the instruction stream
                res.push(instr);

                // Update the input bytes
                input = &input[2..];
            }
            _ => return Err(DecoderError::UnknownInstruction(input[1]).into()),
        }
    }

    Ok(res)
}
