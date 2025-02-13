use rand::Rng;
use std::f64::consts::PI;
use std::ops::RangeInclusive;

fn sin_orig(val: f64) -> f64 {
    val.sin()
}

fn sin_approx(x: f64) -> f64 {
    -4.0 / PI.powi(2) * x.powi(2) + 4.0 / PI * x
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
    let mut res: f64 = val;
    unsafe {
        core::arch::asm!(
            "sqrtsd {0}, {0}", inout(xmm_reg) res, options(nostack)
        );
    }
    res
}

fn sqrt_test2(mut val: f64) -> f64 {
    unsafe {
        core::arch::asm!(
            r#"
            fld qword ptr [{ptr}]
            fsqrt
            fstp qword ptr [{ptr}]
            "#,
            ptr = inout(reg) &mut val => _,
            options(nostack)
        );
    }
    val
}

struct MathTest {
    range: RangeInclusive<f64>,
    control: fn(f64) -> f64,
    tests: &'static [(&'static str, fn(f64) -> f64)],
}

fn main() {
    const NUM_SAMPLES: usize = 100_000_000;
    let sin_range = -3.1415927..=PI;
    let cos_range = -3.1415927..=PI;
    let asin_range = 0.0..=1.0;
    let sqrt_range = 0.0..=1.0;
    let mut rng = rand::thread_rng();

    let tests = [
        MathTest {
            range: 0.0..=PI,
            control: sin_orig,
            tests: &[("sin_approx", sin_approx)],
        },
        MathTest {
            range: sin_range,
            control: sin_orig,
            tests: &[("sin_approx", sin_approx)],
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
            range: sqrt_range.clone(),
            control: sqrt_orig,
            tests: &[("sqrt_test", sqrt_test)],
        },
        MathTest {
            range: sqrt_range,
            control: sqrt_orig,
            tests: &[("sqrt_test2", sqrt_test2)],
        },
    ];

    for MathTest {
        range,
        control,
        tests,
    } in tests
    {
        let mut largest_diff = 0.0;
        let mut largest_diff_input = 0.0;

        for (test_name, test_fn) in tests {
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

            let reference = control(largest_diff_input);
            println!("-- {range:?} --");
            println!("Largest diff for {test_name}");
            let func = format!("  f({largest_diff_input})");
            println!("{func} = {reference} [reference]");

            for (test_name, test_fn) in tests {
                let result = test_fn(largest_diff_input);
                let diff_from_reference = result - reference;
                println!(
                    "{:width$} = {largest_diff_input} ({diff_from_reference}) [{test_name}]",
                    "",
                    width = func.len()
                );
            }

            // Spacing
            println!();
        }
    }
}
