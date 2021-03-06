#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, s: usize, d: usize, ab: [(usize, usize); n]);
    for i in 0..n {
        if ab[i].0 < s && ab[i].1 > d {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
