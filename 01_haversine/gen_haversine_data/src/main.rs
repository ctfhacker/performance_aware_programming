use clap::Parser;
use rand::Rng;
use serde::{Deserialize, Serialize};

mod rng;

#[derive(Parser)]
struct CommandLineArgs {
    /// Number of pairs to randomly generate
    number: usize,

    /// Seed used for the random number generator
    seed: u64,

    /// The minimum X axis to generate
    #[arg(long, allow_negative_numbers(true), default_value_t = -90.0)]
    min_x: f32,

    /// The maximum X axis to generate
    #[arg(long, allow_negative_numbers(true), default_value_t = 90.0)]
    max_x: f32,

    /// The minimum Y axis to generate
    #[arg(long, allow_negative_numbers(true), default_value_t = -180.0)]
    min_y: f32,

    /// The maximum Y axis to generate
    #[arg(long, allow_negative_numbers(true), default_value_t = 180.0)]
    max_y: f32,
}

/// A set of points on earth
#[derive(Serialize, Deserialize)]
struct Points {
    pairs: Vec<Pair>,
}

impl Points {
    pub fn haversine(&self) -> f32 {
        let earth_radius_km = 6371.0;
        let mut sum = 0.0;
        for pair in &self.pairs {
            sum += haversine_degrees(pair, earth_radius_km);
        }

        sum
    }
}

/// A point on the earth
#[derive(Serialize, Deserialize)]
struct Pair {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
}

/// Calculate the haversine distance between (x0, y0) and (x1, y1) assuming
/// the given `radius`
fn haversine_degrees(pair: &Pair, radius: f32) -> f32 {
    let dy = (pair.y1 - pair.y0).to_radians();
    let dx = (pair.x1 - pair.x0).to_radians();
    let y0 = pair.y0.to_radians();
    let y1 = pair.y1.to_radians();

    let root = (dy / 2.0).sin().powi(2) + y0.cos() * y1.cos() * (dx / 2.0).sin().powi(2);
    2.0 * radius * root.sqrt().asin()
}

fn main() {
    let args = CommandLineArgs::parse();

    // Initialize the pairs of points
    let mut pairs = Vec::new();

    // Init the RNG used to generate random points
    let mut rng = rng::Rng::from_seed(args.seed);

    println!(
        "Generating {} random points from range ({}..{}, ({}..{}))",
        args.number, args.min_x, args.max_x, args.min_y, args.max_y
    );

    let mut chunks = Vec::new();
    for _ in 0..16 {
        let mut min_x = rng.gen_range(args.min_x..args.max_x);
        let mut max_x = rng.gen_range(args.min_x..args.max_x);
        if min_x > max_x {
            std::mem::swap(&mut min_x, &mut max_x);
        }
        let mut min_y = rng.gen_range(args.min_y..args.max_y);
        let mut max_y = rng.gen_range(args.min_y..args.max_y);
        if min_y > max_y {
            std::mem::swap(&mut min_y, &mut max_y);
        }

        chunks.push((min_x, max_x, min_y, max_y));
    }

    // Generate random points
    for i in 0..args.number {
        let (min_x, max_x, min_y, max_y) = chunks[rng.next() as usize % chunks.len()];

        #[cfg(debug_assertions)]
        let point = match i {
            // Diagonal
            0 => Pair {
                x0: args.min_x,
                y0: args.min_y,
                x1: args.max_x,
                y1: args.max_y,
            },
            // Left edge
            1 => Pair {
                x0: args.min_x,
                y0: args.min_y,
                x1: args.min_x,
                y1: args.max_y,
            },
            // Bottom edge
            2 => Pair {
                x0: args.min_x,
                y0: args.min_y,
                x1: args.max_x,
                y1: args.min_y,
            },
            // Right edge
            3 => Pair {
                x0: args.max_x,
                y0: args.min_y,
                x1: args.max_x,
                y1: args.max_y,
            },
            // Top edge
            4 => Pair {
                x0: args.min_x,
                y0: args.max_y,
                x1: args.max_x,
                y1: args.max_y,
            },
            // Diagonal
            5 => Pair {
                x0: args.min_x,
                y0: args.max_y,
                x1: args.max_x,
                y1: args.min_y,
            },
            _ => Pair {
                x0: rng.gen_range(min_x..max_x),
                y0: rng.gen_range(min_y..max_y),
                x1: rng.gen_range(min_x..max_x),
                y1: rng.gen_range(min_y..max_y),
            },
        };

        #[cfg(not(debug_assertions))]
        let point = Pair {
            x0: rng.gen_range(min_x..max_x),
            y0: rng.gen_range(min_y..max_y),
            x1: rng.gen_range(min_x..max_x),
            y1: rng.gen_range(min_y..max_y),
        };

        pairs.push(point);
    }

    let points = Points { pairs };
    let haversine = points.haversine() / args.number as f32;

    println!("Haversine: {haversine}");

    // Write the generated JSON file out to disk
    let mut json = serde_json::to_string(&points).expect("Failed to create output JSON");

    // Attempt to match the same format shown by Casey
    json = json.replace('[', "[\n");
    json = json.replace("},", "},\n");
    json = json.replace("{\"x", "    {\"x");

    #[cfg(debug_assertions)]
    let outfile = format!(
        "data_WITH_MINX_{}_MINY_{}_MAXX_{}_MAXY_{}_{}_seed_{}_{haversine}.json",
        args.min_x, args.min_y, args.max_x, args.max_y, args.number, args.seed
    );

    #[cfg(not(debug_assertions))]
    let outfile = format!("data_{}_seed_{}_{haversine}.json", args.number, args.seed);

    println!("Generated file written to {outfile}");
    std::fs::write(&outfile, json).expect("Failed to write {outfile}");
}
