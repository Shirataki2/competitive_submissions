#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: i32, m: i32);
    println!("{}", n * (n - 1) / 2 + m * (m - 1) / 2);
}
