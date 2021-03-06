#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    //! 初回得点 a, 目標 k点とすると
    //! a * 2^n >= k を満たす最小のn を求めればよい
    //! O(log N)で愚直に求めて間に合う．
    input!(n: usize, k: usize);
    let mut ans = 0.0;
    for i in 1..=n {
        let mut p = 1.0 / n as f64;
        let mut point = i;
        while point < k {
            point *= 2;
            p /= 2.0;
        }
        ans += p;
    }
    println!("{}", ans);
}
