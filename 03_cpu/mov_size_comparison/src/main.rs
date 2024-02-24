#![feature(thread_id_value)]

use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, Instant};

use rand::Rng;
use timeloop::RepititionTester;

use mov_size_comparison::*;

pub struct TestParameters {
    /// The allocation we are writing to
    pub buffer: Vec<u8>,

    /// The number of iterations to perform the loop
    pub count: u64,
}

const ALLOC_SIZE: usize = 1024 * 1024 * 1024;
const TEST_DURATION: Duration = Duration::from_millis(250);

#[cfg(feature = "timeloop")]
fn main() {
    println!("Running WITH timeloop...");

    let mut rng = rand::thread_rng();

    let mut buffer = vec![0_u8; ALLOC_SIZE];
    rng.fill(&mut buffer[..]);

    let buffer = buffer;

    let mut results: HashMap<&str, String> = HashMap::new();

    for _ in 0..1 {
        for (func, func_name) in FUNCS.iter() {
            println!("----- {:?} -----", func_name);

            let mut tester = RepititionTester::new(TEST_DURATION);

            while tester.is_testing() {
                // Start the timer for this iteration
                tester.start();

                // Execute the function in question
                (*func)(buffer.as_slice());

                // Stop the timer for this iteration
                tester.stop();

                // Reset the buffer to be reused again
                // params.buffer = vec![0_u8; ALLOC_SIZE];
            }

            let res = tester.results_with_throughput(ALLOC_SIZE as u64);

            let mut iter = func_name.split('x');
            let heading = iter.next().unwrap();
            let num = iter.next().unwrap();

            results.entry(heading).or_default().push_str(&format!(
                "{num} {:8.2}\n",
                res.min.bytes_per_second.unwrap() / 1024. / 1024. / 1024.
            ));
        }
    }

    let mut plot = String::new();

    plot.push_str("set terminal png size 1920,1080\n");
    plot.push_str("set output 'data.png'\n");
    plot.push_str("set title 'MOV width comparisons'\n");
    plot.push_str("set xlabel 'Number of movs'\n");
    plot.push_str("set ylabel 'Throughput GB/s'\n");
    plot.push_str("set xtics (1, 2, 3, 4, 5, 6)\n");

    plot.push_str("plot ");

    // Create the data directory if it doesn't exist
    let data_dir = Path::new("data");
    if !data_dir.exists() {
        std::fs::create_dir(&data_dir).expect("Failed to create data directory");
    }

    let mut plots = Vec::new();
    for (name, res) in results {
        let name = Path::new("data").join(name);
        std::fs::write(&name, res).unwrap();

        plots.push(format!(
            "'{}' with linespoints",
            name.as_os_str().to_str().unwrap()
        ));
    }

    plots.sort();

    plot.push_str(&plots.join(", \\\n"));
    plot.push('\n');

    // plot.push_str("pause -1\n");

    std::fs::write("data.plot", plot).unwrap();
}

/// Calculate the OS frequency by timing a small timeout using `rdtsc`
fn calculate_os_frequency() -> f64 {
    let timeout = Duration::from_millis(100);
    let start = Instant::now();
    let clock_start = unsafe { core::arch::x86_64::_rdtsc() };
    while start.elapsed() < timeout {}
    let clock_end = unsafe { core::arch::x86_64::_rdtsc() };

    (clock_end - clock_start) as f64 / timeout.as_secs_f64()
}

#[cfg(not(feature = "timeloop"))]
fn main() {
    println!("Running manual WITHOUT timeloop...");

    let mut rng = rand::thread_rng();

    const ALLOC_SIZE: usize = 1024 * 1024 * 1024;

    let mut buffer = vec![0_u8; ALLOC_SIZE];
    rng.fill(&mut buffer[..]);

    let buffer = buffer;

    let mut results: HashMap<&str, String> = HashMap::new();
    let mut timers = [u64::MAX; FUNCS.len()];

    let os_freq = calculate_os_frequency();

    for _ in 0..1 {
        for (index, (func, func_name)) in FUNCS.iter().enumerate() {
            println!("----- {:?} -----", func_name);

            let start = Instant::now();

            while start.elapsed() < TEST_DURATION {
                let start = unsafe { core::arch::x86_64::_rdtsc() };

                // Execute the function in question
                (*func)(buffer.as_slice());

                let elapsed = unsafe { core::arch::x86_64::_rdtsc() } - start;

                // Keep the minimum time spent
                timers[index] = elapsed.min(timers[index]);
            }

            dbg!(&timers[index]);

            let mut iter = func_name.split('x');
            let heading = iter.next().unwrap();
            let num = iter.next().unwrap();

            let elapsed_time_sec = timers[index] as f64 / os_freq;
            let gbytes_per_sec = ALLOC_SIZE as f64 / elapsed_time_sec / 1024. / 1024. / 1024.;

            results
                .entry(heading)
                .or_default()
                .push_str(&format!("{num} {:8.2}\n", gbytes_per_sec));

            timers[index] = 0;
        }
    }

    let mut plot = String::new();

    plot.push_str("set terminal png size 1920,1080\n");
    plot.push_str("set output 'data.png'\n");
    plot.push_str("set title 'MOV width comparisons'\n");
    plot.push_str("set xlabel 'Number of movs'\n");
    plot.push_str("set ylabel 'Throughput GB/s'\n");
    plot.push_str("set xtics (1, 2, 3, 4, 5, 6)\n");

    plot.push_str("plot ");

    // Create the data directory if it doesn't exist
    let data_dir = Path::new("data");
    if !data_dir.exists() {
        std::fs::create_dir(&data_dir).expect("Failed to create data directory");
    }

    let mut plots = Vec::new();
    for (name, res) in results {
        let name = Path::new("data").join(name);
        std::fs::write(&name, res).unwrap();

        plots.push(format!(
            "'{}' with linespoints",
            name.as_os_str().to_str().unwrap()
        ));
    }

    plots.sort();

    plot.push_str(&plots.join(", \\\n"));
    plot.push('\n');

    // plot.push_str("pause -1\n");

    std::fs::write("data.plot", plot).unwrap();
}
