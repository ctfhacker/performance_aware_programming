//! An 8086 emulator

#![feature(variant_count)]

use anyhow::Result;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::{Duration, Instant};

use crate::instruction::Instruction;

mod decoder;
mod instruction;
mod memory;
mod register;

#[derive(Debug)]
enum Stats {
    ReadInput,
    Decode,
    WriteDecode,
}

// Read the time stamp using rdtscp to ensure previous instructions have been executed
fn rdtsc() -> u64 {
    let mut x = 0;
    unsafe { std::arch::x86_64::__rdtscp(&mut x) }
}

fn main() -> Result<()> {
    // Attempt to write the CPU speed if we know about it
    if let Ok(speed) = sys_info::cpu_speed() {
        let mut unit = "MHz";
        let mut speed = speed as f64;
        if speed > 1000. {
            speed /= 1000.0;
            unit = "GHz";
        }
        println!("CPU Speed: {speed} {unit}");
    }

    // Read the input file to decode
    let input_file = std::env::args()
        .nth(1)
        .expect("USAGE: ./emu8086 <8086_File>");

    // Set the output file
    let output_file = Path::new(&input_file).with_extension(".rebuilt.decoded");

    // Init statistics for this performance check
    let mut stats = [0u64; std::mem::variant_count::<Stats>()];
    let mut stats_time = [Duration::from_secs(0); std::mem::variant_count::<Stats>()];

    let mut best_stats = [u64::MAX; std::mem::variant_count::<Stats>()];
    let mut best_stats_time = [Duration::from_secs(u64::MAX); std::mem::variant_count::<Stats>()];

    const ITERS: usize = 1000;

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
                best_stats[Stats::$stat as usize] = curr_stats_cycle
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
    for _ in 0..ITERS {
        // Read the input bytes from the input file
        let bytes = time!(ReadInput, std::fs::read(&input_file)?);

        // Decode the input byte stream
        let instrs = time!(Decode, decoder::decode_stream(&bytes)?);

        // Print the decoded instructions
        time!(WriteDecode, {
            let mut file = File::create(&output_file)?;

            file.write(format!("; Decoded from {input_file}\n").as_bytes())?;
            file.write_all(b"bits 16\n")?;
            for instr in instrs {
                if matches!(instr, Instruction::Lock) {
                    file.write(format!("{instr}").as_bytes())?;
                } else {
                    file.write(format!("{instr}\n").as_bytes())?;
                }
            }
        });
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
                "{:014} | Best iter {:>8.2?} Avg {:>8.2?}/iter | Best {:10.2?} Avg {:10.2?} cycles/iter | {:>5.2}% total time",
                format!("{:?}", Stats::$stat),
                best_stat_time,
                Duration::from_nanos((curr_stat_time.as_nanos() as f64 / ITERS as f64) as u64),
                best_stat,
                curr_stat as f64 / ITERS as f64,
                curr_stat as f64 / total_elapsed as f64 * 100.
            );
        }};
    }

    print_stat!(ReadInput);
    print_stat!(Decode);
    print_stat!(WriteDecode);

    Ok(())
}
