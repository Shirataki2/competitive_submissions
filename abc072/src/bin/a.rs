#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(x: i64, t: i64);
    println!("{}", max(0, x - t));
}
