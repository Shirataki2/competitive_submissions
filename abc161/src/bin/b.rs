#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, m: usize, mut a: [usize; n]);
    a.sort();
    a.reverse();
    println!("{}", if a[m-1] * 4 * m < a.iter().sum() { "No" } else { "Yes" });
}
