#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(h: u64, a: u64);
    println!("{}", (h + a - 1) / a);
}
