#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

fn price(a: i64, b: i64, n: i64) -> i64 {
    let digits = {
        let mut n = n;
        let mut ctr = 0;
        while n > 0 { ctr += 1; n /= 10; }
        ctr
    };
    a * n + b * digits
}

#[fastout]
fn main() {
    input!(a: i64, b: i64, x: i64);
    let mut left = 0;
    let mut right = 1_000_000_001;
    for _ in 0..1000 {
        let mid = (left + right) / 2;
        if price(a, b, mid) <= x {
            left = mid;
        } else {
            right = mid;
        }
    }
    println!("{}", left);
}
