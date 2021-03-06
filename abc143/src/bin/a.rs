#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(a: i64, b: i64);
    println!("{}", max(0, a - 2 * b));
}
