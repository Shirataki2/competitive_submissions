#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(h: i64, n: usize, a: [i64; n]);
    println!("{}", if a.iter().sum::<i64>() >= h { "Yes" } else { "No" });
}
