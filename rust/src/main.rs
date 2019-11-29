extern crate num_bigint_dig as num_bigint;
extern crate num_traits;
use async_std::task;
use async_std::prelude::*;
use std::mem::replace;
use std::task::{Context, Poll};
use std::pin::Pin;
use std::future::Future;
use futures;
use std::time::Instant;
use num_bigint::BigUint;
use num_traits::{Zero, One};

fn main() {
    let size = 220000;
    let start = Instant::now();
    run_fib(2000, size);
    let end = Instant::now();
    let duration = end.duration_since(start);
    let ns = duration.as_nanos();
    println!("fib_2k    {} ns/op", ns / 220000);

    let size = 200000;
    let start = Instant::now();
    run_fib(3000, size);
    let end = Instant::now();
    let duration = end.duration_since(start);
    let ns = duration.as_nanos();
    println!("fib_3k    {} ns/op", ns / 200000);

    let size = 15000;
    let start = Instant::now();
    run_fib(10000, size);
    let end = Instant::now();
    let duration = end.duration_since(start);
    let ns = duration.as_nanos();
    println!("fib_10k    {} ns/op", ns / 15000);

    let size = 10000;
    let start = Instant::now();
    run_fib(20000, size);
    let end = Instant::now();
    let duration = end.duration_since(start);
    let ns = duration.as_nanos();
    println!("fib_20k    {} ns/op", ns / 10000);

    let size = 2500;
    let start = Instant::now();
    run_fib(40000, size);
    let end = Instant::now();
    let duration = end.duration_since(start);
    let ns = duration.as_nanos();
    println!("fib_40k    {} ns/op", ns / 2500);

    let size = 2500;
    let start = Instant::now();
    run_fib(50000, size);
    let end = Instant::now();
    let duration = end.duration_since(start);
    let ns = duration.as_nanos();
    println!("fib_50k    {} ns/op", ns / 2500);
}

fn run_fib(n: usize, size: usize) {
    task::block_on(async {
        let tasks = (0..size).into_iter().map(|BigUint| task::spawn(fib(n)));
        futures::future::join_all(tasks).await;
    })
}

async fn fib(n: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        f0 = replace(&mut f1, f2);
    }
    f0
}

// old implementation
// fn main2() {
//     let start = SystemTime::now();
//     fib_thread(2000);
//     let end = SystemTime::now();
//     let duration = end.duration_since(start)
//         .expect("Time went backwards");
//     let ns = duration.as_nanos();
//     println!("fib_2k    {} ns/op", ns/2000);
// }

fn fib_thread(n: usize) {
    let (tx, rx) = mpsc::channel();
    for _ in 0..2000 {
        let tx = tx.clone();
        thread::spawn(move || {
            fib(n);
            tx.send(());
        });
    }
    for _ in 0..2000 {
        rx.recv().ok();
    }
}
