//! An 8086 emulator

#![deny(missing_docs)]
#![feature(stdsimd)]
#![feature(variant_count)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use anyhow::Result;
use emu::{Core, JitEmulatorState};
use jit::JitBuffer;

use std::arch::asm;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::{Duration, Instant};

use cpu8086::emu::Emulator;
use cpu8086::instruction::Instruction;

#[derive(Debug)]
enum Stats {
    ReadInput,
    Decode,
    WriteDecode,
    BuildJit,
    ExecJit,
}

/// Read the time stamp using rdtscp to ensure previous instructions have been executed
fn rdtsc() -> u64 {
    let mut x = 0;
    unsafe { std::arch::x86_64::__rdtscp(&mut x) }
}

// Attempt to write the CPU speed if we know about it
#[allow(clippy::cast_precision_loss)]
fn print_cpu_speed() {
    if let Ok(speed) = sys_info::cpu_speed() {
        let mut unit = "MHz";
        let mut speed = speed as f64;
        if speed > 1000. {
            speed /= 1000.0;
            unit = "GHz";
        }
        println!("CPU Speed: {speed} {unit}");
    }
}

/// Number of iterations to measure the test with
const ITERS: usize = 0x1;

#[allow(clippy::too_many_lines)]
fn main() -> Result<()> {
    // Print CPU speed of the processor running the emulator
    print_cpu_speed();

    let term_width = 110;

    // Read the input file to decode
    let input_file = std::env::args()
        .nth(1)
        .expect("USAGE: ./emu8086 <8086_File>");

    // Set the output file
    let output_file = Path::new(&input_file).with_extension("rebuilt.decoded.asm");

    // Init statistics for this performance check
    let mut stats = [0u64; std::mem::variant_count::<Stats>()];
    let mut stats_time = [Duration::from_secs(0); std::mem::variant_count::<Stats>()];

    let mut best_stats = [u64::MAX; std::mem::variant_count::<Stats>()];
    let mut best_stats_time = [Duration::from_secs(u64::MAX); std::mem::variant_count::<Stats>()];

    println!("Number of iterations: {ITERS:#x}");

    let total_start = rdtsc();

    /// Macro used for timing work
    macro_rules! time {
        ($stat:ident, $work:expr) => {{
            // Start the timer
            let start_time = Instant::now();
            let start = rdtsc();

            // Perform the work being timed
            let res = $work;

            // Add the elapsed time this work took
            let curr_stats_cycle = rdtsc() - start;

            // Update the clock time stats
            let curr_stats_time = start_time.elapsed();

            stats[Stats::$stat as usize] += curr_stats_cycle;

            // If this is the best stats time, update it.
            if best_stats[Stats::$stat as usize] > curr_stats_cycle {
                best_stats[Stats::$stat as usize] = curr_stats_cycle;
            }

            stats_time[Stats::$stat as usize] += curr_stats_time;

            // If this is the fastest clock time seen, update it
            if best_stats_time[Stats::$stat as usize] > curr_stats_time {
                best_stats_time[Stats::$stat as usize] = curr_stats_time;
            }

            // Return the result from the work
            res
        }};
    }

    // Run the tests over a number of iterations in order to average the time
    let mut file = File::create(&output_file)?;

    file.write_all(format!("; Decoded from {input_file}\n").as_bytes())?;
    file.write_all(b"bits 16\n")?;

    let debug_on = false;

    // Main iteration loop
    for _iter in 0..ITERS {
        // Init the emulator
        let mut emu = time!(
            ReadInput,
            Emulator::<1024>::with_memory(Path::new(&input_file))?
        );

        let mut jit = JitBuffer::<{ 1024 * 1024 }>::new();

        for _iter in 0.. {
            // If we've read past the end of the emulator, return..
            if emu.registers.ip as usize >= emu.memory.length {
                break;
            }

            let ip = emu.registers.ip;

            // Decode the input byte stream
            let decoded_instr = time!(
                Decode,
                cpu8086::decoder::decode_instruction(&mut emu.registers, &emu.memory).unwrap()
            );

            // Print the decoded instructions
            time!(WriteDecode, {
                if matches!(decoded_instr, Instruction::Lock) {
                    file.write_all(format!("{decoded_instr}").as_bytes())?;
                } else {
                    file.write_all(format!("{decoded_instr}\n").as_bytes())?;
                }
            });

            // Cache the starting offset for the next JIT instructions
            let curr_offset = jit.offset;

            // Cache the formatted decoded instruction string
            let decoded_instr_str = format!("{decoded_instr}");

            // JIT the decoded instruction to the JIT stream
            time!(BuildJit, {
                // Get the JIT instruction for the decoded 8086 instruction
                jit.write_instr(decoded_instr);
            });

            // Debug print the JIT assembly for the decoded instruction
            let jit_instr = jit.get_disassembly_between(curr_offset, jit.offset);
            for (i, line) in jit_instr.iter().enumerate() {
                if i == 0 {
                    println!("{ip:#05x} {decoded_instr_str:20} | {line}");
                } else {
                    println!("{:26} | {line}", "");
                }
            }
            println!("{}", "-".repeat(60));
        }

        // Initialize the JIT emulator
        let jit_emu = JitEmulatorState::default();

        #[allow(clippy::cast_possible_truncation)]
        let core = rdtsc() as u8 % 20 + 1;

        println!("+{:-^width$}+", " CPU Before ", width = term_width - 2);

        jit_emu.print_cpu_state(Core(core));

        // Execute the JIT buffer
        time!(ExecJit, {
            unsafe {
                asm!(include_str!("../.tmp_files/findme.rs"),
                    in("r13") usize::from(debug_on),
                    in("r14") jit.buffer() as usize,
                    in("r15") &jit_emu,
                );
            }
        });

        println!("+{:-^width$}+", " CPU After ", width = term_width - 2);

        jit_emu.print_cpu_state(Core(core));
    }

    // Stop the clock on the entire work load
    let total_elapsed = rdtsc() - total_start;

    /// Macro used to pretty print the statistics
    macro_rules! print_stat {
        ($stat:ident) => {{
            let curr_stat = stats[Stats::$stat as usize];
            let curr_stat_time = stats_time[Stats::$stat as usize];
            let best_stat = best_stats[Stats::$stat as usize];
            let best_stat_time = best_stats_time[Stats::$stat as usize];

            eprintln!(
                // "{:12} \n  Best {:<8.2?} \n  Avg  {:<.2?}/iter \n  Best {:<.2?} cycles/iter \n  Avg  {:<.2?} cycles/iter \n  % of total time: {:5.2}%",
                "{:12} | Best {:>8.2?} | Avg {:>8.2?}/iter | Best {:>8.2?} cycles/iter | Avg {:>10.2?} cycles/iter | % of total time: {:5.2}%",
                format!("{:?}", Stats::$stat),
                best_stat_time,
                Duration::from_nanos((curr_stat_time.as_nanos() as f64 / ITERS as f64) as u64),
                best_stat,
                curr_stat as f64 / ITERS as f64,
                curr_stat as f64 / total_elapsed as f64 * 100.
            );
        }};
    }

    println!(
        "+{:-^width$}+",
        " Performance Stats ",
        width = term_width - 2
    );

    print_stat!(ReadInput);
    print_stat!(Decode);
    print_stat!(WriteDecode);
    print_stat!(BuildJit);
    print_stat!(ExecJit);

    Ok(())
}
