#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    let mut v = vec![false; 101];
    for i in 1..=9 {
        for j in 1..=9 {
            v[i*j] |= true;
        }
    }
    input!(n: usize);
    println!("{}", if v[n] { "Yes" } else { "No" })
}
