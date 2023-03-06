//! An 8086 emulator

use anyhow::Result;

mod decoder;
mod instruction;
mod memory;
mod register;

fn main() -> Result<()> {
    let input_file = std::env::args()
        .nth(1)
        .expect("USAGE: ./emu8086 <8086_File>");

    let bytes = std::fs::read(&input_file)?;

    let instrs = decoder::decode_stream(&bytes)?;

    println!("; Decoded from {input_file}");
    println!("bits 16");
    for instr in instrs {
        println!("{instr}");
    }

    Ok(())
}
