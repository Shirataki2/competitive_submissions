#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;
use std::collections::*;


#[fastout]
fn main() {
    input!(n: usize, m: usize);
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        input!(a: usize, b: usize);
        g[a-1].push(b-1);
        g[b-1].push(a-1);
    }
    input!(k: usize, mut c: [usize; k]);
    c.iter_mut().for_each(|ci| *ci -= 1);

    // k点間での全点間最短経路を求める
    let mut costs = vec![];
    for s in c.iter() {
        let mut dij = BinaryHeap::<(usize, usize)>::new();
        let mut cost = vec![1<<60; n];
        dij.push((*s, 0));
        cost[*s as usize] = 0;
        while !dij.is_empty() {
            let (v, cur) = dij.pop().unwrap();
            if cost[v] < cur { continue; }
            for u in g[v].iter() {
                if chmin!(cost[*u], cur + 1) {
                    dij.push((*u, cur + 1));
                }
            }
        }
        let mut cc = vec![];
        for ci in c.iter() {
            cc.push(cost[*ci]);
        }

        costs.push(cc);
    }

    // TSP
    let mut dp = vec![vec![1i64<<60; k]; 1<<k];
    dp[1][0] = 0;
    for s in 0..1<<k {
        for v in 0..k {
            for u in 0..k {
                if s & (1 << u) > 0 { continue; }
                chmin!(dp[s | (1 << u)][u], dp[s][v] + costs[v][u] as i64);
            }
        }
    }
    let ans = dp[(1<<k)-1].iter().min().copied().unwrap();
    println!("{}", if ans == 1i64<<60 { -1 } else { ans + 1 });
}

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

