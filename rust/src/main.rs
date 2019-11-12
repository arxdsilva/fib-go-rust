use rayon::prelude::*;
use std::mem::replace;
use std::time::Instant;

#[inline(never)]
fn fib(n: usize) -> u128 {
    let mut f0 = 0u128;
    let mut f1 = 1u128;
    for _ in 0..n {
        let f2 = f0 + f1;
        f0 = replace(&mut f1, f2);
    }
    f0
}

fn main() {
    let start = Instant::now();
    fib_thread_pool(2000);
    let end = Instant::now();
    let duration = end.duration_since(start);
    let ns = duration.as_nanos();
    println!("fib_2k    {} ns/op", ns / 2000);

    let start = Instant::now();
    fib_thread_pool(3000);
    let end = Instant::now();
    let duration = end.duration_since(start);
    let ns = duration.as_nanos();
    println!("fib_3k    {} ns/op", ns / 2000);

    let start = Instant::now();
    fib_thread_pool(10000);
    let end = Instant::now();
    let duration = end.duration_since(start);
    let ns = duration.as_nanos();
    println!("fib_10k    {} ns/op", ns / 2000);

    let start = Instant::now();
    fib_thread_pool(20000);
    let end = Instant::now();
    let duration = end.duration_since(start);
    let ns = duration.as_nanos();
    println!("fib_20k    {} ns/op", ns / 2000);

    let start = Instant::now();
    fib_thread_pool(40000);
    let end = Instant::now();
    let duration = end.duration_since(start);
    let ns = duration.as_nanos();
    println!("fib_40k    {} ns/op", ns / 2000);
}

fn fib_thread_pool(n: usize) {
    (0..2000)
        .into_par_iter()
        .map(|_| fib(n))
        .collect::<Vec<_>>();
}
