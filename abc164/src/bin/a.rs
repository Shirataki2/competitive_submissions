#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(s: i32, w: i32);
    println!("{}", if s > w { "safe" } else { "unsafe" });
}
