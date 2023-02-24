use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::Instant;

use anyhow::Result;
use clap::Parser;
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

/// Calculate the haversine distance between (x0, y0) and (x1, y1) assuming
/// the given `radius`
fn haversine_degrees(pair: Pair, radius: f32) -> f32 {
    let dy = (pair.y1 - pair.y0).to_radians();
    let dx = (pair.x1 - pair.x0).to_radians();
    let y0 = pair.y0.to_radians();
    let y1 = pair.y1.to_radians();

    let root = (dy / 2.0).sin().powi(2) + y0.cos() * y1.cos() * (dx / 2.0).sin().powi(2);
    2.0 * radius * root.sqrt().asin()
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

    let data = simd_data;

    // Time the math calculation
    let start_time = Instant::now();
    let earth_radius_km = 6371.0;
    let mut sum = 0.0;
    let mut count = 0;
    for pair in data.pairs.iter() {
        sum += haversine_degrees(*pair, earth_radius_km);
        count += 1;
    }
    let math_time = Instant::now() - start_time;
    let total_time = math_time + simd_time;
    let average = sum / count as f32;
    let math_percent = math_time.as_secs_f32() / total_time.as_secs_f32() * 100.;
    let simd_percent = simd_time.as_secs_f32() / total_time.as_secs_f32() * 100.;
    let throughput = count as f32 / total_time.as_secs_f32() / 1_000_000.;

    println!("Result     = {average:4.2}");
    println!(
        "Input      = {simd_time:10.4?} | {simd_percent:6.2?}% of total time (using simdjson)"
    );
    println!(
        "Math       = {math_time:10.4?} | {math_percent:6.2?}% of total time (using simdjson)"
    );
    println!("Total      = {total_time:8.4?} seconds (using simdjson)");
    println!("Throughput = {throughput:4.2} Mhaversines/seconds");

    Ok(())
}
