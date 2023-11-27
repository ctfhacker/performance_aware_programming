#![feature(thread_id_value)]

use std::time::Duration;

use rand::Rng;
use timeloop::RepititionTester;

use front_end_test::*;

pub struct TestParameters {
    /// The allocation we are writing to
    pub buffer: Vec<u8>,

    /// The number of iterations to perform the loop
    pub count: u64,
}

fn main() {
    let mut rng = rand::thread_rng();

    let funcs: &mut [(&'static str, fn(&mut [u8]))] = &mut [
        ("MovAllbytes", test_mov_all_bytes),
        ("ThreeByteNopAllbytes", test_three_byte_nop_all_bytes),
        ("3_SingleByteNopAllbytes", test_3_single_byte_nop_all_bytes),
        ("1_SingleByteNopAllbytes", test_1_single_byte_nop_all_bytes),
        ("CmpAllBytes", test_cmp_all_bytes),
        ("DecAllBytes", test_dec_all_bytes),
    ];

    const ALLOC_SIZE: usize = 1024 * 1024 * 1024;

    let mut buffer = vec![0_u8; ALLOC_SIZE];
    rng.fill(&mut buffer[..]);

    for _ in 0..3 {
        // Randomly choose which function to test
        // funcs.shuffle(&mut rng);

        for func in funcs.iter() {
            let mut tester = RepititionTester::new(Duration::from_secs(2));

            while tester.is_testing() {
                // Start the timer for this iteration
                tester.start();

                // Execute the function in question
                func.1(buffer.as_mut_slice());

                // Stop the timer for this iteration
                tester.stop();

                // Reset the buffer to be reused again
                // params.buffer = vec![0_u8; ALLOC_SIZE];
            }

            println!("----- {} -----", func.0);
            tester.results.print_with_bytes(ALLOC_SIZE as u64);
        }
    }
}
