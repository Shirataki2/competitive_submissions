#![allow(unused_imports)]
use proconio::{input, fastout};
use itertools_num::ItertoolsNum;
use std::cmp::*;

#[fastout]
fn main() {
    input!(k: u64, n: usize, mut a: [u64; n]);
    a.push(k + a[0]);
    let mut ans = std::u64::MAX;
    for i in 0..n {
        ans = min(ans, k - (a[i+1] - a[i]));
    }
    println!("{}", ans);
}
