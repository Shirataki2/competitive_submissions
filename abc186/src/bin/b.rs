#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(h: usize, w: usize, a: [u64; h * w]);
    let min_a = a.iter().min().unwrap();
    println!("{}", a.iter().fold(0, |acc, &x| acc + x - min_a));
}
