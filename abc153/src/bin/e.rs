#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[macro_export]
macro_rules! min {
    ($a: expr) => { $a };
    ($a: expr, $b: expr) => { std::cmp::min($a, $b) };
    ($a: expr, $($rest: expr),+) => { std::cmp::min($a, min!($($rest),+)) };
}

#[macro_export]
macro_rules! chmin {
    ($a: expr, $($rest: expr),+) => {{
        let cmp_min = min!($($rest),+);
        if $a > cmp_min { $a = cmp_min; true } else { false }
    }};
}

#[fastout]
fn main() {
    //! dp[i] モンスターの体力をi減らすのに必要な魔力の最小値
    //! dp[i] = min_{j}(dp[i-a[j]] + b[j])
    const MAX: usize = 10_000;
    input!(h: usize, n: usize);
    let mut a = vec![0; n];
    let mut b = vec![0; n];
    for i in 0..n {
        input!(x: usize, y: i64);
        a[i] = x;
        b[i] = y;
    }
    let mut dp = vec![1<<60; MAX+1];
    dp[0] = 0;
    for i in 0..=MAX {
        for j in 0..n {
            if i >= a[j] {
                chmin!(dp[i], dp[i - a[j]] + b[j]);
            }
        }
    }
    let ans = dp[h..].iter().min().unwrap();
    println!("{}", ans);
}
