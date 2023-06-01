use clap::Parser;
use std::path::PathBuf;

mod json;

#[derive(Debug)]
enum Error {
    Io(std::io::Error),
    Json(json::JsonError),
}

/// Calculate the haversine distance for the given
#[derive(Parser, Debug)]
struct Args {
    /// Input file containing json haversine points
    input: PathBuf,

    /// The pre-calculated answer for the given input
    answer: Option<PathBuf>,
}

/// Calculate the haversine distance between (x0, y0) and (x1, y1) assuming
/// the given `radius`
fn haversine_degrees(x0: f64, y0: f64, x1: f64, y1: f64, radius: f64) -> f64 {
    let dy = (y1 - y0).to_radians();
    let dx = (x1 - x0).to_radians();
    let y0 = y0.to_radians();
    let y1 = y1.to_radians();

    let root = (dy / 2.0).sin().powi(2) + y0.cos() * y1.cos() * (dx / 2.0).sin().powi(2);
    2.0 * radius * root.sqrt().asin()
}

const EARTH_RADIUS_KM: f64 = 6371.0;
const ONE_GB: usize = 1024 * 1024 * 1024;
const ONE_MB: usize = 1024 * 1024;
const ONE_KB: usize = 1024;

/// Format a given number of bytes to a human readable format
#[allow(clippy::cast_precision_loss)]
fn format_bytes(num: usize) -> String {
    if num >= ONE_GB {
        format!("{:.2} GB", num as f64 / ONE_GB as f64)
    } else if num >= ONE_MB {
        format!("{:.2} MB", num as f64 / ONE_MB as f64)
    } else if num >= ONE_KB {
        format!("{:.2} KB", num as f64 / ONE_KB as f64)
    } else {
        format!("{num} bytes")
    }
}

fn main() -> Result<(), Error> {
    // Parse the command line arguments
    let args = Args::parse();

    // Read the given input
    let data = std::fs::read_to_string(&args.input).map_err(Error::Io)?;
    println!("Input size: {} ({})", data.len(), format_bytes(data.len()));

    // Get the answer file or look for a `.answer` file from the input `.json` file
    let answer = args
        .answer
        .or({
            let answer = args.input.with_extension("answer");
            if answer.exists() {
                Some(answer)
            } else {
                None
            }
        })
        .map(|x| {
            println!("Using answer file: {x:?}");
            std::fs::read_to_string(x)
        })
        .map(|x| x.unwrap().parse::<f64>());

    // Parse the given data using the json parser
    let data = json::parse(&data).map_err(Error::Json)?;

    // Retrieve the data from the parsed JSON
    let pairs = data["pairs"].as_vec().map_err(Error::Json)?;
    println!("Pair count: {}", pairs.len());

    // Calculate the haversine over the parsed pairs
    let mut sum = 0.0;
    for pair in pairs {
        let pair = pair.as_map().map_err(Error::Json)?;
        let x0 = pair["x0"].as_num().map_err(Error::Json)?;
        let x1 = pair["x1"].as_num().map_err(Error::Json)?;
        let y0 = pair["y0"].as_num().map_err(Error::Json)?;
        let y1 = pair["y1"].as_num().map_err(Error::Json)?;
        let haversine = haversine_degrees(*x0, *y0, *x1, *y1, EARTH_RADIUS_KM);
        sum += haversine;
    }

    // Calculate the average among the given pairs
    sum /= pairs.len() as f64;

    println!("Haversine: {sum:24.20}");

    if let Some(Ok(answer)) = answer {
        let diff = sum - answer;
        println!("--- Validation ---");
        println!("Answer:    {answer:24.20}");
        println!("Difference: {diff:24.20}");
    }

    Ok(())
}
