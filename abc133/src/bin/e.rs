#![allow(unused_imports)]
use proconio::{input, fastout, marker::Usize1};
use std::cmp::*;

type Edge = usize;
type Graph = Vec<Vec<Edge>>;

const MOD: usize = 1_000_000_007;

fn dfs(v: usize, p: Option<usize>, k: usize, g: &Graph) -> usize {
    // 親p = Noneは根ということにしとく
    let mut available_color = match p {
        Some(_) => k - 2,
        None    => k - 1,
    };
    let c = match p {
        Some(_) => g[v].len() - 1,
        None    => g[v].len(),
    };
    let mut permutation = {
        let mut permutation = 1;
        for _ in 0..c {
            permutation *= available_color;
            available_color -= 1;
            permutation %= MOD;
        }
        permutation
    };
    for &u in g[v].iter() {
        if let Some(from) = p {
            if from == u {
                continue;
            }
        }
        permutation *= dfs(u, Some(v), k, &g);
        permutation %= MOD;
    }
    permutation
}

//#[fastout]
fn main() {
    //! 根はK色どれでも塗れる
    //! 根から生えてる子の塗り分け方は子の数がcであれば k-1_P_c
    //! 根じゃない頂点から生えてる子は親を考慮する必要があるので k-2_P_c
    //! 根からDFS的に掛けていけばよさそう
    input!(n: usize, k: usize);
    let mut g: Graph = vec![vec![]; n];
    for _ in 0..n-1 {
        input!(a: Usize1, b: Usize1);
        g[a].push(b);
        g[b].push(a);
    }
    let mut ans = k * dfs(0, None, k, &g);
    ans %= MOD;
    println!("{}", ans);
}
