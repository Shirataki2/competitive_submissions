#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, mut k: usize);
    input!(a: [usize; n]);
    // nxt[d][v]: vから2^d回テレポートした後の町
    // Doubling: 
    // nxt[0][v] := a[v]
    // nxt[d][v] := nxt[d-1][nxt[d-1][v]]
    let mut nxt = vec![vec![0; n]; 63];
    for v in 0..n {
        nxt[0][v] = a[v] - 1;
    }
    for d in 1..63 {
        for v in 0..n {
            nxt[d][v] = nxt[d-1][nxt[d-1][v]];
        }
    }
    let mut ans = 0;
    for i in 0..63 {
        if k >> i & 1 == 1 { ans = nxt[i][ans]; }
    }
    println!("{}", ans + 1);
}
