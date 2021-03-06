#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: i64, k: i64);
    println!("{}", min(n % k, k - n % k));
}
