#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

const NULL: usize = std::usize::MAX;

#[derive(Debug, Clone)]
struct Edge {
    to: usize,
    index: usize,
}

fn dfs(v: usize, p: Option<usize>, pc: Option<usize>, g: &Vec<Vec<Edge>>, mut colors: &mut Vec<usize>) {
    let pc = pc.unwrap_or(NULL);
    let mut c = 1;
    if pc == c { c += 1; }
    for e in g[v].iter() {
        if e.to == p.unwrap_or(NULL) { continue; }
        colors[e.index] = c;
        dfs(e.to, Some(v), Some(c), &g, &mut colors);
        c += 1;
        if pc == c { c += 1; }
    }
}

#[fastout]
fn main() {
    input!(n: usize);
    let mut g = vec![vec![]; n];
    let mut colors = vec![0; n-1];
    for i in 0..n-1 {
        input!(mut a: usize, mut b: usize);
        a -= 1; b -= 1;
        g[a].push(Edge { to: b, index: i });
        g[b].push(Edge { to: a, index: i });
    }
    dfs(0, None, None, &g, &mut colors);
    println!("{}", colors.iter().max().unwrap());
    for v in colors.iter() { println!("{}", v); }
}
