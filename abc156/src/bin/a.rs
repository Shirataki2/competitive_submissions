#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: i64, r: i64);
    println!("{}", if n < 10 { r + 100 * (10 - n) } else { r });
}
