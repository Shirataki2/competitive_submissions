#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(v: i64, t: i64, s: i64, d: i64);
    println!("{}", if v * t > d || d > v * s { "Yes" } else { "No" });
}
