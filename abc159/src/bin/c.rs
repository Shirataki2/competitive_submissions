#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(l: f64);
    println!("{}", (l / 3.0).powf(3.0))
}
