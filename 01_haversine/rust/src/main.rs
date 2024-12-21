use std::fs::File;
use std::io::BufReader;
use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use anyhow::Result;
use clap::Parser;
use core_affinity::{set_for_current, CoreId};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
struct CommandLineArgs {
    /// Input file containing pairs of (latitude,longitude) coordinates
    input: PathBuf,
}

/// A set of points on earth
#[derive(Serialize, Deserialize, PartialEq)]
pub struct Points {
    pairs: Vec<Pair>,
}

/// A point on the earth
#[derive(Serialize, Deserialize, PartialEq, Copy, Clone)]
struct Pair {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Sin,
    Cos,
    Asin,
    Sqrt,
}

/// Calculate the haversine distance between (x0, y0) and (x1, y1) assuming
/// the given `radius`
#[cfg(not(feature = "log_bounds"))]
fn haversine_degrees(pair: Pair, radius: f32) -> f32 {
    let dy = (pair.y1 - pair.y0).to_radians();
    let dx = (pair.x1 - pair.x0).to_radians();
    let y0 = pair.y0.to_radians();
    let y1 = pair.y1.to_radians();

    let root = (dy / 2.0).sin().powi(2) + y0.cos() * y1.cos() * (dx / 2.0).sin().powi(2);
    2.0 * radius * root.sqrt().asin()
}

/// Calculate the haversine distance between (x0, y0) and (x1, y1) assuming
/// the given `radius`
#[cfg(feature = "log_bounds")]
fn haversine_degrees(pair: Pair, radius: f32, ranges: &mut [(f32, f32); 4]) -> f32 {
    let dy = (pair.y1 - pair.y0).to_radians();
    let dx = (pair.x1 - pair.x0).to_radians();
    let y0 = pair.y0.to_radians();
    let y1 = pair.y1.to_radians();

    macro_rules! sin {
        ($val:expr) => {{
            ranges[Operation::Sin as usize].0 = ranges[Operation::Sin as usize].0.min($val);
            ranges[Operation::Sin as usize].1 = ranges[Operation::Sin as usize].1.max($val);
            $val.sin()
        }};
    }

    macro_rules! cos {
        ($val:expr) => {{
            ranges[Operation::Cos as usize].0 = ranges[Operation::Cos as usize].0.min($val);
            ranges[Operation::Cos as usize].1 = ranges[Operation::Cos as usize].1.max($val);
            $val.cos()
        }};
    }

    macro_rules! asin {
        ($val:expr) => {{
            ranges[Operation::Asin as usize].0 = ranges[Operation::Asin as usize].0.min($val);
            ranges[Operation::Asin as usize].1 = ranges[Operation::Asin as usize].1.max($val);
            $val.asin()
        }};
    }

    macro_rules! sqrt {
        ($val:expr) => {{
            ranges[Operation::Sqrt as usize].0 = ranges[Operation::Sqrt as usize].0.min($val);
            ranges[Operation::Sqrt as usize].1 = ranges[Operation::Sqrt as usize].1.max($val);
            $val.sqrt()
        }};
    }

    let root = sin!(dy / 2.0).powi(2) + cos!(y0) * cos!(y1) * sin!(dx / 2.0).powi(2);
    2.0 * radius * asin!(sqrt!(root))
}

pub fn serde_json(input: &PathBuf) -> Result<Points> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|e| e.into())
}

pub fn simd_json(input: &PathBuf) -> Result<Points> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    simd_json::serde::from_reader(reader).map_err(|e| e.into())
}

#[cfg(not(feature = "log_bounds"))]
pub fn naive_work(data: Arc<Points>, cores: usize) -> f32 {
    let earth_radius_km = 6371.0;
    let mut sum = 0.0;
    for pair in &data.pairs {
        sum += haversine_degrees(*pair, earth_radius_km);
    }
    sum
}

#[cfg(feature = "log_bounds")]
pub fn naive_work(data: Arc<Points>, cores: usize) -> f32 {
    let earth_radius_km = 6371.0;
    let mut sum = 0.0;
    let mut ranges = [(f32::MAX, f32::MIN); 4];
    for pair in &data.pairs {
        sum += haversine_degrees(*pair, earth_radius_km, &mut ranges);
    }

    println!(
        "{:?}: {:?}",
        Operation::Sin,
        ranges[Operation::Sin as usize]
    );
    println!(
        "{:?}: {:?}",
        Operation::Cos,
        ranges[Operation::Cos as usize]
    );
    println!(
        "{:?}: {:?}",
        Operation::Asin,
        ranges[Operation::Asin as usize]
    );
    println!(
        "{:?}: {:?}",
        Operation::Sqrt,
        ranges[Operation::Sqrt as usize]
    );

    sum
}

#[cfg(not(feature = "log_bounds"))]
pub fn rayon_work_par_iter(data: Arc<Points>, cores: usize) -> f32 {
    std::env::set_var("RAYON_NUM_THREADS", format!("{cores}"));

    let earth_radius_km = 6371.0;
    let mut sum = 0.0;

    let pairs = data.pairs.as_slice();
    let sum: f32 = pairs
        .par_iter()
        .map(|pair| haversine_degrees(*pair, earth_radius_km))
        .sum();

    sum
}

#[cfg(not(feature = "log_bounds"))]
fn worker(data: Arc<Points>, start_index: usize, len: usize) -> f32 {
    let mut sum = 0.0;
    let earth_radius_km = 6371.0;
    for offset in start_index..start_index + len {
        let pair = data.pairs[offset];
        sum += haversine_degrees(pair, earth_radius_km);
    }
    sum
}

/// Manually chunk and parallelize the data for the given number of cores
#[cfg(not(feature = "log_bounds"))]
pub fn manual_chunk_parallel(data: Arc<Points>, cores: usize) -> f32 {
    let per_core_len = data.pairs.len() / cores;
    let last_core_offset = data.pairs.len() - (cores * per_core_len);
    let mut threads = Vec::new();
    let core_ids = core_affinity::get_core_ids().expect("Failed to get core ids");

    for (index, core_id) in core_ids.into_iter().enumerate().take(cores) {
        let offset = index * per_core_len;
        let data = data.clone();
        let mut len = per_core_len - 1;

        // If the data isn't evenly divisible by the number of cores,
        // attempt to equally spread the remainder work between the cores
        if index < last_core_offset {
            len += 1;
        }

        let t = std::thread::spawn(move || {
            // Pin this thread to this core_id
            set_for_current(core_id);

            // Execute the worker thread for this core
            worker(data, offset, len)
        });
        threads.push(t);
    }

    let mut sum = 0.0;
    for t in threads {
        sum += t.join().unwrap()
    }

    sum
}

pub fn main() -> Result<()> {
    let args = CommandLineArgs::parse();

    // Time serde_json
    let start_time = Instant::now();
    let serde_data = serde_json(&args.input)?;
    let serde_time = Instant::now() - start_time;
    println!("Reading via serde_json: {:6.2?}", serde_time);

    // Time simd_json
    let start_time = Instant::now();
    let simd_data = simd_json(&args.input)?;
    let simd_time = Instant::now() - start_time;
    let speedup = serde_time.as_secs_f32() / simd_time.as_secs_f32();
    println!("Reading via simd_json:  {:6.2?}", simd_time);
    println!("simdjson speedup over serde: {speedup:6.4}x");

    // Sanity check that the simdjson and serde_json result in the same data
    assert!(simd_data == serde_data, "serde and simd json disagree!");

    let data = Arc::new(simd_data);
    let count = data.pairs.len();

    // Print the read in
    println!("Input      = {simd_time:10.4?}");

    // Time the given work function parallelized over the given number of cores
    macro_rules! time_work {
        ($work_func:ident, $cores:expr) => {
            // Get a reference to the data for this test (only incrementing a ref counter)
            let data = data.clone();

            // Start the timer for this work
            let start_time = Instant::now();

            // Execute the given tested work
            let sum: f32 = $work_func(data, $cores);

            // Stop the timer
            let math_time = Instant::now() - start_time;

            // Calculate statistics for this test case
            let total_time = simd_time + math_time;
            let math_percent = math_time.as_secs_f32() / total_time.as_secs_f32() * 100.;
            let simd_percent = simd_time.as_secs_f32() / total_time.as_secs_f32() * 100.;
            let average = sum / count as f32;
            let throughput = count as f32 / total_time.as_secs_f32() / 1_000_000.;

            // Print the statistic results
            println!(
                "{} {} with {} cores {}",
                "-".repeat(40 - stringify!($work_func).len() / 2),
                stringify!($work_func),
                $cores,
                "-".repeat(40 - stringify!($work_func).len() / 2),
            );
            println!("{:30} = {average:4.2}", "Average");
            println!(
                "{:30} = {simd_time:10.4?} | {simd_percent:6.2}% of total time (using simdjson)",
                "simdjson"
            );
            println!(
                "{:30} = {math_time:10.4?} | {math_percent:6.2}% of total time (using simdjson)",
                stringify!($work_func),
            );
            println!(
                "{:30} = {total_time:8.4?} seconds (using simdjson)",
                "Total"
            );
            println!("{:30} = {throughput:4.2} Mhaversines/seconds", "Throughput");
        };
    }

    time_work!(naive_work, 1);

    // time_work!(rayon_work_par_iter, 2);
    // time_work!(rayon_work_par_iter, 4);
    // time_work!(rayon_work_par_iter, 8);
    // time_work!(rayon_work_par_iter, 12);
    // time_work!(rayon_work_par_iter, 16);

    // time_work!(manual_chunk_parallel, 2);
    // time_work!(manual_chunk_parallel, 4);
    // time_work!(manual_chunk_parallel, 8);
    // time_work!(manual_chunk_parallel, 12);
    // time_work!(manual_chunk_parallel, 16);

    Ok(())
}
