#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, k: usize);
    let mut ctr = vec![0; n];
    for _ in 0..k {
        input!(d: usize, a: [usize; d]);
        a.iter().for_each(|&ai| ctr[ai-1] += 1);
    }
    println!("{}", ctr.iter().filter(|&c| *c == 0).count());
}
