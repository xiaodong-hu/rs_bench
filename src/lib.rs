use colored::*;
use std::time::Duration;

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
        time_block!($block, "default")
    }};
}

#[macro_export]
macro_rules! time_expr {
    ($expr:expr, $message:expr) => {{
        time_block!(
            {
                $expr;
            },
            $message
        )
    }};
    ($expr:expr) => {{
        time_expr!($expr, "default")
    }};
}
