#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, mut ab: [(i64, i64); n]);
    ab.sort_by_key(|(a, b)| 2 * a + b);
    ab.reverse();
    let mut diff = ab.iter().fold(0, |acc, (a, _)| acc + a);
    for i in 0..n {
        diff -= ab[i].0 * 2 + ab[i].1;
        if diff < 0 {
            println!("{}", i+1);
            return
        }
    }
}
