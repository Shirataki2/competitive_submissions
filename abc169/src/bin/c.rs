#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(a: u64, b: String);
    let b: u64 = b.replace(".", "").parse().unwrap();
    let ans = a * b;
    let n = ans / 100;
    println!("{}", n);
}
