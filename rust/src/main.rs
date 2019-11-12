// #![feature(test)]
extern crate num_bigint_dig as num_bigint;
extern crate num_traits;
// extern crate test;

use std::time::{SystemTime, UNIX_EPOCH};
use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::mem::replace;
// use test::Bencher;
use std::thread;
use std::sync::mpsc;

fn fib(n: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        f0 = replace(&mut f1, f2);
    }
    f0
}

fn main() {
    let start = SystemTime::now();
    fib_thread(2000);
    let end = SystemTime::now();
    let duration = end.duration_since(start)
        .expect("Time went backwards");
    let ns = duration.as_nanos();
    println!("fib_2k    {} ns/op", ns/2000);
    // 3k threads
    let start = SystemTime::now();
    fib_thread(3000);
    let end = SystemTime::now();
    let duration = end.duration_since(start)
        .expect("Time went backwards");
    let ns = duration.as_nanos();
    println!("fib_3k    {} ns/op", ns/2000);
    // 10k threads
    let start = SystemTime::now();
    fib_thread(10000);
    let end = SystemTime::now();
    let duration = end.duration_since(start)
        .expect("Time went backwards");
    let ns = duration.as_nanos();
    println!("fib_10k    {} ns/op", ns/2000);
    // 20k threads
    let start = SystemTime::now();
    fib_thread(20000);
    let end = SystemTime::now();
    let duration = end.duration_since(start)
        .expect("Time went backwards");
    let ns = duration.as_nanos();
    println!("fib_20k    {} ns/op", ns/2000);
    // 40k threads
    let start = SystemTime::now();
    fib_thread(40000);
    let end = SystemTime::now();
    let duration = end.duration_since(start)
        .expect("Time went backwards");
    let ns = duration.as_nanos();
    println!("fib_40k    {} ns/op", ns/2000);
    // fib sequential
    // 2k threads
    let start = SystemTime::now();
    fib_seq(2000, 25000);
    let end = SystemTime::now();
    let duration = end.duration_since(start)
        .expect("Time went backwards");
    let ns = duration.as_nanos();
    println!("fib_seq_2k    {} ns/op", ns/25000);
    // 3k threads
    let start = SystemTime::now();
    fib_seq(3000, 14000);
    let end = SystemTime::now();
    let duration = end.duration_since(start)
        .expect("Time went backwards");
    let ns = duration.as_nanos();
    println!("fib_seq_3k    {} ns/op", ns/14000);
    // 10k threads
    let start = SystemTime::now();
    fib_seq(10000, 2500);
    let end = SystemTime::now();
    let duration = end.duration_since(start)
        .expect("Time went backwards");
    let ns = duration.as_nanos();
    println!("fib_seq_10k    {} ns/op", ns/2500);
    // 20k threads
    let start = SystemTime::now();
    fib_seq(20000, 900);
    let end = SystemTime::now();
    let duration = end.duration_since(start)
        .expect("Time went backwards");
    let ns = duration.as_nanos();
    println!("fib_seq_20k    {} ns/op", ns/900);
    // 40k threads
    let start = SystemTime::now();
    fib_seq(40000, 250);
    let end = SystemTime::now();
    let duration = end.duration_since(start)
        .expect("Time went backwards");
    let ns = duration.as_nanos();
    println!("fib_seq_40k    {} ns/op", ns/250);
}

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


fn fib_seq(n: usize, size: usize) {
    for _ in 0..size {
        fib(n);
    }
}

// #[bench]
// fn bench_fib_3k(b: &mut Bencher) {
//     b.iter(|| {
//         let (tx, rx) = mpsc::channel();
//         for _ in 0..2000 {
//             let tx = tx.clone();
//             thread::spawn(move || {
//                 fib(3000);
//                 tx.send(());
//             });
//         }
//         for _ in 0..2000 {
//             rx.recv();	
//         }
//     });
// }

// #[bench]
// fn bench_fib_10k(b: &mut Bencher) {
//     b.iter(|| {
//         let (tx, rx) = mpsc::channel();
//         for _ in 0..2000 {
//             let tx = tx.clone();
//             thread::spawn(move || {
//                 fib(10000);
//                 tx.send(());
//             });
//         }
//         for _ in 0..2000 {
//             rx.recv();	
//         }
//     });
// }

// #[bench]
// fn bench_fib_20k(b: &mut Bencher) {
//      b.iter(|| {fib_thread(20000)});
// }

// #[bench]
// fn bench_fib_40k(b: &mut Bencher) {
//      b.iter(|| {fib_thread(40000)});
// }
