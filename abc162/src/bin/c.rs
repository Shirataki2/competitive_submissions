#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(k: u64);
    let mut ans = 0;
    for a in 1..=k {
        for b in 1..=k {
            for c in 1..=k {
                ans += gcd(gcd(a, b), c)
            }
        }
    }
    println!("{}", ans);
}

pub fn gcd(a: u64, b: u64) -> u64 {
    match (a, b) {
        (a, 0) => a,
        (a, b) => gcd(b, a % b)
    }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

pub fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b > 0 {
        let (gcd, mut y, x) = extgcd(b, a % b);
        y -= (a / b) * x;
        (gcd, x, y)
    } else {
        (a, 1, 0)
    }
}