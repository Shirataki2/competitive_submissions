#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, a: [i64; n], b: [i64; n]);
    println!("{}", if a.iter().zip(b.iter()).fold(0, |acc, (&a, &b)| acc + a * b) == 0 { "Yes" } else { "No" });
}
