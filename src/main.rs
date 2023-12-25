#![feature(duration_abs_diff)]

mod lib;
use lib::*;

const N_DROP_MEASUREMENT: usize = 5; // number of measurements to drop (due to warm-up)
const N_MEASUREMENT: usize = 5; // 5 is enough if the first several fluctuating measurements are dropped!

fn dot_test(v1: &[f64], v2: &[f64]) {
    let res = v1.iter().zip(v2.iter()).map(|(&a, &b)| a * b).sum::<f64>();
    // dbg!(res);
}

fn main() {
    let mut vec1 = Vec::new();
    for i in 1..100000 {
        vec1.push((i as f64).sqrt())
    }
    let vec2 = vec1.clone();

    time_expr![dot_test(&vec1, &vec2), "this is me"];
    time_block![{ dot_test(&vec1, &vec2) }, "this is me"]
}
