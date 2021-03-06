#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, d: [usize; n]);
    let mut ans = 0;
    for i in 0..n-1 {
        for j in i+1..n {
            ans += d[i] * d[j];
        }
    }
    println!("{}", ans);
}
