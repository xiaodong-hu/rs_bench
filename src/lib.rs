#![feature(duration_abs_diff)]

use colored::*;
use std::time::Duration;

const N_MEASREMENT: usize = 50;
pub struct Benchmark {
    time_vec: Vec<std::time::Instant>,
    pub message: &'static str,
}
impl Benchmark {
    pub fn new(message: &'static str) -> Self {
        Benchmark {
            time_vec: vec![std::time::Instant::now(); 2 * N_MEASREMENT],
            message: message,
        }
    }
}
impl Drop for Benchmark {
    fn drop(&mut self) {
        let time_elapse_vec = self
            .time_vec
            .iter()
            .map(|t| t.elapsed())
            .collect::<Vec<_>>();
        let time_elapse_mean = time_elapse_vec
            .iter()
            .sum::<Duration>()
            .div_f64(time_elapse_vec.len() as f64);
        let time_elapse_variance_f64 = (time_elapse_vec
            .iter()
            .map(|t| t.abs_diff(time_elapse_mean).as_secs_f64().powf(2.0))
            .sum::<f64>()
            / ((time_elapse_vec.len() - 1) as f64))
            .sqrt();

        let time_elapse_variance = {
            let time_elapse_variance_sec = time_elapse_variance_f64 as u64;
            let time_elapse_variance_nano = ((time_elapse_variance_f64
                - time_elapse_variance_sec as f64)
                * 1000_000_000 as f64) as u32;

            Duration::new(time_elapse_variance_sec, time_elapse_variance_nano)
        };

        let time_elapse_mean = format!("{:?}", time_elapse_mean);
        let time_elapse_variance = format!("{:?}", time_elapse_variance);
        match self.message.len() {
            0 => println!(
                "{} {} {} {}",
                "Task takes".bold(),
                time_elapse_mean.bold().red(),
                "±".bold(),
                time_elapse_variance.bold().red()
            ),
            _ => println!(
                "{} `{}` {} {} {} {}",
                "Task".bold(),
                self.message,
                "takes".bold(),
                time_elapse_mean.bold().red(),
                "±".bold(),
                time_elapse_variance.bold().red()
            ),
        }
    }
}

#[macro_export]
macro_rules! time_block {
    // Case for benchmarking with a custom message
    ($block:block, $message:expr) => {{
        let _benchmark = Benchmark::new($message);
        $block
    }};
    // Case for benchmarking with empty message
    ($block:block) => {{
        let _benchmark = Benchmark::new("");
        $block
    }};
}

#[macro_export]
macro_rules! time_expr {
    // Case for benchmarking with a custom message
    ($expr:expr, $message:expr) => {{
        let _benchmark = Benchmark::new($message);
        $expr
    }};
    // Case for benchmarking without a custom message
    ($expr:expr) => {{
        let _benchmark = Benchmark::new("");
        $expr
    }};
}
