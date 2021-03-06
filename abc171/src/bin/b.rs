use proconio::{input, fastout};
#[allow(unused_imports)]
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, k: usize, mut p: [u32; n]);
    p.sort();
    println!("{}", (0..k).fold(0, |acc, i| acc + p[i]));
}
