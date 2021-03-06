#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(x: u64);
    let ans = 1000 * (x / 500) + 5 * ((x % 500) / 5);
    println!("{}", ans);
}
