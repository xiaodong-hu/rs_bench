#![feature(duration_abs_diff)]

mod lib;
use lib::*;

fn test() {
    let mut vec1 = Vec::new();
    for i in 1..1000 {
        vec1.push(i)
    }
    let vec2 = vec1.clone();
    let res = vec1
        .iter()
        .zip(vec2.iter())
        .fold(0, |acc, (a, b)| acc + a * b);

    dbg!(res);
}

fn main() {
    time_block![{ test() }]
}
