#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

fn digits(s: &str) -> Vec<u32> {
    s.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

#[fastout]
fn main() {
    input!(a: String, b: String);
    println!("{}", max::<u32>(digits(&a).iter().sum(), digits(&b).iter().sum()));
}
