#![allow(unused_imports)]
use proconio::{input, fastout, marker::*};
use std::cmp::*;
use std::collections::*;

#[derive(Debug, Clone)]
struct Edge {
    to: usize,
    cost: i64,
    intv: i64,
}

const MAX: i64 = std::i64::MAX / 2;

//#[fastout]
fn main() {
    input!(n: usize, m: usize, x: Usize1, y: Usize1);
    let mut g: Vec<Vec<Edge>> = vec![vec![]; n];
    for _ in 0..m {
        input!(a: Usize1, b: Usize1, t: i64, k: i64);
        g[a].push(Edge { to: b, cost: t, intv: k });
        g[b].push(Edge { to: a, cost: t, intv: k });
    }

    // 二分木ヒープは最大値から出される！！！！！！！！
    let mut heap = BinaryHeap::new();
    let mut dists = vec![MAX; n];
    dists[x] = 0;
    // (距離, 頂点)
    heap.push(Reverse((0_i64, x)));
    while !heap.is_empty() {
        let (d, v) = heap.pop().unwrap().0;
        if dists[v] < d { continue; }
        for edge in g[v].iter() {
            if chmin!(dists[edge.to], (dists[v] - 1).div_euclid(edge.intv) * edge.intv + edge.intv + edge.cost) {
                heap.push(Reverse((dists[edge.to], edge.to)));
            }
        }
    }

    println!("{}", if dists[y] < MAX { dists[y] } else { -1 });
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