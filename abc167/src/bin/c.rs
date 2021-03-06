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
    input!(n: usize, m: usize, x: i64);
    let mut costs = vec![0; n];
    let mut skills = vec![vec![0; m]; n];
    for i in 0..n {
        input!(v: i64);
        costs[i] = v;
        for j in 0..m {
            input!(v: i64);
            skills[i][j] = v;
        }
    }
    let mut ans = std::i64::MAX;
    for i in 0..1<<n {
        let mut current_skills = vec![0; m];
        let mut current_cost = 0;
        for bit in 0..n {
            if i >> bit & 1 == 1{
                current_cost += costs[bit];
                for j in 0..m {
                    current_skills[j] += skills[bit][j];
                }
            }
        }
        if current_skills.iter().all(|&v| v >= x) {
            chmin!(ans, current_cost);
        }
    }
    if ans == std::i64::MAX {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
}
