#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;
use std::collections::*;


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

fn dfs(a: &Vec<i64>, g: &Vec<Vec<usize>>, i: usize, mut dp: &mut Vec<i64>, mut visited: &mut Vec<bool>) {
    visited[i] = true;
    for &to in g[i].iter() {
        if visited[to] { continue }
        dfs(a, g, to, &mut dp, &mut visited);
        chmax!(dp[to], dp[i], a[i]);
    }
}

#[fastout]
fn main() {
    input!(n: usize, m: usize);
    input!(a: [i64; n]);
    let mut graph = vec![vec![]; n];
    let mut dp = vec![0; n];
    let mut visited = vec![false; n];
    for _ in 0..m {
        input!(mut x: usize, mut y: usize);
        x -= 1; y -= 1;
        graph[x].push(y);
    }
    for i in 0..n {
        if !visited[i] {
            dfs(&a, &graph, i, &mut dp, &mut visited);
        }
    }
    println!("{}", dp.iter().max().unwrap());
}
