use colored::*;
use std::time::Duration;

// use rayon::prelude::*;

pub const N_DROP_MEASUREMENT: usize = 10; // number of measurements to drop (due to warm-up)
pub const N_MEASUREMENT: usize = 5; // 10 is enough if the first several fluctuating measurements are dropped!

pub fn time_elapse_statistics(time_measurements: &[Duration]) -> (Duration, f64) {
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

    return (mean_time, std_deviation);
}

#[macro_export]
macro_rules! time_block {
    ($block:block, $message:expr) => {{
        let (mut mean_time,mut std_deviation) = (std::time::Duration::new(1,0),1.0);
        let mut drop_shift = 1; // automatically adjust the number of dropped measurements
        while std_deviation/mean_time.as_secs_f64() > 0.3 {
            let mut time_measurements = [std::time::Duration::default(); N_MEASUREMENT];
            for i in 0..(drop_shift + N_DROP_MEASUREMENT + N_MEASUREMENT) {
                let start = std::time::Instant::now();
                $block
                if i >= drop_shift + N_DROP_MEASUREMENT {
                    time_measurements[i - N_DROP_MEASUREMENT - drop_shift] = start.elapsed();
                }
            }
            // dbg!(time_measurements);
            (mean_time, std_deviation) = time_elapse_statistics(&time_measurements);
            drop_shift += 2;
        }

        dbg!(drop_shift);
        let mean_time = format!("{:?}", mean_time);
        let std_deviation = format!("{:?}", Duration::from_secs_f64(std_deviation));
        println!(
            "{} `{}` {} {} {} {}",
            "Task".bold(),
            $message.italic().bold(),
            "takes".bold(),
            mean_time.bold().red(),
            "±".bold(),
            std_deviation.bold().red()
        );
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
