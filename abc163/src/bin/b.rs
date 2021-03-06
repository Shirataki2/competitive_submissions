#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: i64, m: usize, a: [i64; m]);
    let ans = n - a.iter().sum::<i64>();
    println!("{}", if ans < 0 { -1 } else { ans });
}
