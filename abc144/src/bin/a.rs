#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(a: i64, b: i64);
    println!("{}", if a > 9 || b > 9 { -1 } else { a * b });
}
