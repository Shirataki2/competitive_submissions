#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

fn main() {
    input!(l: u64, r: u64);
    let mut ans = 2019;
    'outer: for i in l..=r-1 {
        for j in l+1..=r {
            chmin!(ans, (i*j)%2019);
            if ans == 0 { break 'outer; }
        }
    }
    println!("{}", ans);
}

#[macro_export]
macro_rules! min {
    ($a: expr) => { $a };
    ($a: expr, $b: expr) => { std::cmp::min($a, $b) };
    ($a: expr, $($rest: expr),+) => { std::cmp::min($a, min!($($rest),+)) };
}

#[macro_export]
macro_rules! chmin {
    ($a: expr, $($rest: expr),+) => {{
        let cmp_min = min!($($rest),+);
        if $a > cmp_min { $a = cmp_min; true } else { false }
    }};
}
