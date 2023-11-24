# rs_bench
simple function call and expression block benchmark macro

### Usage
```rust
fn test(n: i32) {
    let mut vec1 = Vec::new();
    for i in 1..n {
        vec1.push(i)
    }
    let vec2 = vec1.clone();
    let res = vec1
        .iter()
        .zip(vec2.iter())
        .fold(0, |acc, (a, b)| acc + a * b);

    dbg!(res);
}

benchmark!(test(1000), "this is me");
/*
gives [src/lib.rs:69] res = 332833500
Task `this is me` takes 111.813µs
*/
```
