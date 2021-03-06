#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: u64, a: u64, b: u64);
    let c = a + b;
    println!("{}", a * (n / c) + min(a, n % c));
}
