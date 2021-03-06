#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[macro_export]
macro_rules! max {
    ($a: expr) => { $a };
    ($a: expr, $b: expr) => { std::cmp::max($a, $b) };
    ($a: expr, $($rest: expr), +) => { std::cmp::max($a, max!($($rest),+)) };
}

#[macro_export]
macro_rules! chmax {
    ($a: expr, $($rest: expr),+) => {{
        let cmp_max = max!($($rest),+);
        if $a < cmp_max { $a = cmp_max; true } else { false }
    }};
}

#[fastout]
fn main() {
    input!(n: usize, m: usize, h: [i64; n]);
    let mut max_h = vec![0; n];
    for _ in 0..m {
        input!(mut a: usize, mut b: usize);
        a -= 1; b -= 1;
        chmax!(max_h[a], h[b]);
        chmax!(max_h[b], h[a]);
    }
    println!("{}", h.iter().zip(max_h.iter()).filter(|(&hi, &mi)| hi > mi).count());
}
