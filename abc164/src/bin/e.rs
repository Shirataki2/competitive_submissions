#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;
use std::collections::BinaryHeap;

#[derive(Debug, Clone)]
struct Vertex { index: usize, sliver: i64, time: i64 }

#[derive(Debug, Clone)]
struct Edge { to: usize, time: i64, cost: i64 }

#[fastout]
fn main() {
    //! 銀貨をs'枚持ってる時の頂点を動的に生やせば
    //! ダイクストラ法が使える
    //! 
    //! (銀貨はN*max(A) (高々2500)で足りる)
    input!(n: usize, m: usize, mut s: i64);
    let mut times = vec![vec![1<<60; 2501]; n];
    chmin!(s, 2500);
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        input!(u: usize, v: usize, a: i64, b: i64);
        let u = u - 1;
        let v = v - 1;
        graph[u].push(Edge { to: v, time: b, cost: a });
        graph[v].push(Edge { to: u, time: b, cost: a });
    }
    let mut vertices = Vec::with_capacity(n);
    for i in 0..n {
        input!(c: i64, d: i64);
        vertices.push(Vertex { index: i, sliver: c, time: d });
    }

    let mut heap = BinaryHeap::<Reverse<(i64, usize, i64)>>::new();
    heap.push(Reverse((0, 0, s))); // time, index, silver
    times[0][s as usize] = 0;
    while !heap.is_empty() {
        let Reverse((time, index, silver)) = heap.pop().unwrap();
        if time > times[index][silver as usize] { continue; }
        // 銀を取得
        if silver + vertices[index].sliver <= 2500 {
            let n_silver = vertices[index].sliver + silver;
            let n_time = vertices[index].time + time;
            if chmin!(times[index][n_silver as usize], n_time) {
                heap.push(Reverse((n_time, index, n_silver)));
            }
        }
        // 辺を辿る
        for edge in graph[index].iter() {
            if silver < edge.cost { continue; }
            let u = edge.to;
            let n_silver = silver - edge.cost;
            let n_time = time + edge.time;
            if chmin!(times[u][n_silver as usize], n_time) {
                heap.push(Reverse((n_time, u, n_silver)));
            }
        }
    }
    for v in 1..n {
        println!("{}", times[v].iter().min().unwrap());
    }
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