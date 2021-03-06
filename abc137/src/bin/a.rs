#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(a: i16, b: i16);
    println!("{}", max!(a + b, a - b, a * b));
}

#[macro_export]
macro_rules! max {
    ($a: expr) => { $a };
    ($a: expr, $b: expr) => { std::cmp::max($a, $b) };
    ($a: expr, $($rest: expr), +) => { std::cmp::max($a, max!($($rest),+)) };
}
