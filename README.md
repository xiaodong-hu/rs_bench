# rs_bench
simple function call and expression block benchmark macro (after dropping of first five fluctuating measurements)

### Usage
```rust
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

    time_expr![dot_test(&vec1, &vec2), "this is me"]; // works for one-line function calls
    time_block![{ dot_test(&vec1, &vec2) }, "this is me"] // works for mearsurment of blocks
}

/*
Task `this is me` takes 32ns ± 2ns
Task `this is me` takes 31ns ± 1ns
*/
```
