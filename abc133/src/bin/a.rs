#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: i64, a: i64, b: i64);
    println!("{}", min(a*n, b));
}
