#![feature(test)]
extern crate num_bigint_dig as num_bigint;
extern crate num_traits;
extern crate test;

use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::mem::replace;
use test::Bencher;
use std::thread;
use std::sync::mpsc;


fn main() {
    println!("fib(1000) = {}", fib(1000));
}

fn fib(n: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        f0 = replace(&mut f1, f2);
    }
    f0
}


#[bench]
fn bench_fib(b: &mut Bencher) {
    b.iter(|| {
        let mut range = 0..2000;
        let (tx, rx) = mpsc::channel();
        match range.next() {
            thread::spawn(move || {
                fib(1000);
                tx.send(());
            });
        }
        match range.next() {
            rx.recv();
        }
    });
}
