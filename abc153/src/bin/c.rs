#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, k: usize, mut h: [i64; n]);
    h.sort();
    h.reverse();
    if k >= n {
        println!("{}", 0);
    } else {
        let ans: i64 = h[k..].iter().sum();
        println!("{}", ans);
    }
}
