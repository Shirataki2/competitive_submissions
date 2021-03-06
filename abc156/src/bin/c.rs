#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, x: [i64; n]);
    let ans = (1..=100)
        .map(
            |p| 
                x
                    .iter()
                    .fold(0, |acc, &x| acc + (x - p).pow(2))
        )
        .min()
        .unwrap();
    println!("{}", ans);
}
