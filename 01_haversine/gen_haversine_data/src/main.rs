use clap::Parser;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
struct CommandLineArgs {
    /// Number of pairs to randomly generate
    number: usize,

    /// The minimum value to generate
    #[arg(long, allow_negative_numbers(true), default_value_t = -200.0)]
    min: f32,

    /// The maximum value to generate
    #[arg(long, allow_negative_numbers(true), default_value_t = 200.0)]
    max: f32,
}

/// A point on the earth
#[derive(Serialize, Deserialize)]
struct Points {
    pairs: Vec<Point>,
}

/// A point on the earth
#[derive(Serialize, Deserialize)]
struct Point {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
}

fn main() {
    let args = CommandLineArgs::parse();

    // Initialize the pairs of points
    let mut pairs = Vec::new();

    // Init the RNG used to generate random points
    let mut rng = rand::thread_rng();

    // Get the minimum and maximum values from the command line
    let min = args.min;
    let max = args.max;

    println!(
        "Generating {} random points from range ({min}..{max})",
        args.number
    );

    // Generate 10M random points
    for _ in 0..args.number {
        let point = Point {
            x0: rng.gen_range(min..max),
            y0: rng.gen_range(min..max),
            x1: rng.gen_range(min..max),
            y1: rng.gen_range(min..max),
        };

        pairs.push(point);
    }

    let points = Points { pairs };

    // Write the generated JSON file out to disk
    let mut json = serde_json::to_string(&points).expect("Failed to create output JSON");

    // Attempt to match the same format shown by Casey
    json = json.replace("[", "[\n");
    json = json.replace("},", "},\n");
    json = json.replace("{\"x", "    {\"x");

    let outfile = format!("data_{}_flex.json", args.number);
    println!("Generated file written to {outfile}");
    std::fs::write(&outfile, json).expect("Failed to write {outfile}");
}
