use colored::*;
use std::time::Duration;

// use rayon::prelude::*;

const N_DROP_MEASUREMENT: usize = 5; // number of measurements to drop (due to warm-up)
const N_MEASUREMENT: usize = 10; // 10 is enough if the first several fluctuating measurements are dropped!

pub fn time_elapse_statistics(time_measurements: &[Duration], info: &str) {
    let total_time: Duration = time_measurements.iter().sum();
    let mean_time = total_time / (time_measurements.len() as u32);
    let variance: f64 = time_measurements
        .iter()
        .map(|&duration| {
            let diff = duration.as_secs_f64() - mean_time.as_secs_f64();
            diff * diff
        })
        .sum::<f64>()
        / time_measurements.len() as f64;
    let std_deviation = variance.sqrt();

    let mean_time = format!("{:?}", mean_time);
    let std_deviation = format!("{:?}", Duration::from_secs_f64(std_deviation));
    println!(
        "{} `{}` {} {} {} {}",
        "Task".bold(),
        info.italic().bold(),
        "takes".bold(),
        mean_time.bold().red(),
        "±".bold(),
        std_deviation.bold().red()
    );
}

#[macro_export]
macro_rules! time_block {
    ($block:block, $message:expr) => {{
        use crate::N_MEASUREMENT;
        use crate::N_DROP_MEASUREMENT;
        let mut time_measurements = [std::time::Duration::default(); N_MEASUREMENT];
        for i in 0..(N_DROP_MEASUREMENT + N_MEASUREMENT) {
            let start = std::time::Instant::now();
            $block
            if i >= N_DROP_MEASUREMENT {
                time_measurements[i - N_DROP_MEASUREMENT] = start.elapsed();
            }
        }
        // dbg!(time_measurements);
        time_elapse_statistics(&time_measurements, $message);
    }};
    ($block:block) => {{
        time_block![$block, "default"]
    }};
}

#[macro_export]
macro_rules! time_expr {
    ($expr:expr, $message:expr) => {{
        time_block![
            {
                $expr;
            },
            $message
        ]
    }};
    ($expr:expr) => {{
        time_expr!($expr, "default")
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dot_test(v1: &[f64], v2: &[f64]) {
        let res = v1.iter().zip(v2.iter()).map(|(&a, &b)| a * b).sum::<f64>();
        // dbg!(res);
    }

    #[test]
    fn it_works() {
        let mut vec1 = Vec::new();
        for i in 1..100000 {
            vec1.push((i as f64).sqrt())
        }
        let vec2 = vec1.clone();

        time_expr![dot_test(&vec1, &vec2), "this is me"];
        time_block![{ dot_test(&vec1, &vec2) }, "this is me"]
    }
}
