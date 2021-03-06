#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[macro_export]
macro_rules! max {
    ($a: expr) => { $a };
    ($a: expr, $b: expr) => { std::cmp::max($a, $b) };
    ($a: expr, $($rest: expr), +) => { std::cmp::max($a, max!($($rest),+)) };
}

#[macro_export]
macro_rules! chmax {
    ($a: expr, $($rest: expr),+) => {{
        let cmp_max = max!($($rest),+);
        if $a < cmp_max { $a = cmp_max; true } else { false }
    }};
}

fn dfs(n: usize, m: i64, req: &[(usize, usize, i64, i64)], mut a: &mut Vec<i64>, mut ans: &mut i64) {
    if a.len() == n {
        let mut p = 0;
        for r in req.iter() {
            if a[r.1-1] - a[r.0-1] == r.2 {
                p += r.3;
            }
        }
        chmax!(*ans, p);
    }
    else {
        for i in a.last().copied().unwrap_or(1)..=m {
            a.push(i);
            dfs(n, m, req, &mut a, &mut ans);
            a.pop();
        }
    }
}

#[fastout]
fn main() {
    input!(n: usize, m: i64, q: usize, req: [(usize, usize, i64, i64); q]);
    let mut a = vec![];
    let mut ans = 0;
    dfs(n, m, &req, &mut a, &mut ans);
    println!("{}", ans);
}
