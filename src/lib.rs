use colored::*;

pub struct Benchmark {
    time: std::time::Instant,
    pub message: &'static str,
}
impl Benchmark {
    pub fn new(message: &'static str) -> Self {
        Benchmark {
            time: std::time::Instant::now(),
            message: message,
        }
    }
}
impl Drop for Benchmark {
    fn drop(&mut self) {
        let time_elapse = format!("{:?}", self.time.elapsed());
        match self.message.len() {
            0 => println!("{} {}", "Task takes".bold(), time_elapse.bold().red()),
            _ => println!(
                "{} `{}` {} {}",
                "Task".bold(),
                self.message,
                "takes".bold(),
                time_elapse.bold().red()
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
