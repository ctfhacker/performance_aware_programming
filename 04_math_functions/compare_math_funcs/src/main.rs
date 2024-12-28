use rand::Rng;
use std::ops::RangeInclusive;

fn sin_orig(val: f64) -> f64 {
    val.sin()
}

fn sin_test(val: f64) -> f64 {
    val.sin()
}

fn cos_orig(val: f64) -> f64 {
    val.cos()
}

fn cos_test(val: f64) -> f64 {
    val.cos()
}

fn asin_orig(val: f64) -> f64 {
    val.asin()
}

fn asin_test(val: f64) -> f64 {
    val.asin()
}

fn sqrt_orig(val: f64) -> f64 {
    val.sqrt()
}

fn sqrt_test(val: f64) -> f64 {
    val.sqrt()
}

struct MathTest {
    range: RangeInclusive<f64>,
    control: fn(f64) -> f64,
    tests: &'static [(&'static str, fn(f64) -> f64)],
}

fn main() {
    const NUM_SAMPLES: usize = 1_000_000;
    let sin_range = -3.1415927..=3.1415927;
    let cos_range = -3.1415927..=3.1415927;
    let asin_range = 0.0..=1.0;
    let sqrt_range = 0.0..=1.0;
    let mut rng = rand::thread_rng();

    let tests = [
        MathTest {
            range: sin_range,
            control: sin_orig,
            tests: &[("sin_test", sin_test)],
        },
        MathTest {
            range: cos_range,
            control: cos_orig,
            tests: &[("cos_test", cos_test)],
        },
        MathTest {
            range: asin_range,
            control: asin_orig,
            tests: &[("asin_test", asin_test)],
        },
        MathTest {
            range: sqrt_range,
            control: sqrt_orig,
            tests: &[("sqrt_test", sqrt_test)],
        },
    ];

    for MathTest {
        range,
        control,
        tests,
    } in tests
    {
        for (test_name, test_fn) in tests {
            let mut largest_diff = 0.0;
            let mut largest_diff_input = 0.0;

            for _ in 0..NUM_SAMPLES {
                let val: f64 = rng.gen_range(range.clone());
                let control_res = control(val);
                let test_res = test_fn(val);

                let diff = (control_res - test_res).abs();
                if diff > largest_diff {
                    largest_diff = diff;
                    largest_diff_input = val;
                }
            }

            println!("-- {test_name} --");
            println!("Largest diff: {largest_diff}");
            println!("Largest diff input: {largest_diff_input}");
        }
    }
}
