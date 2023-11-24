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
        let time_elapse = format!("takes {:?}", self.time.elapsed());
        match self.message.len() {
            0 => println!("{} {}", "Task".bold(), time_elapse.bold().red()),
            _ => println!(
                "{} `{}` {}",
                "Task".bold(),
                self.message,
                time_elapse.bold().red()
            ),
        }
    }
}

#[macro_export]
macro_rules! benchmark_block {
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
macro_rules! benchmark {
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

fn example() {
    let mut vec1 = Vec::new();
    for i in 1..1000 {
        vec1.push(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        benchmark![example(), "this is me"]
    }
}
