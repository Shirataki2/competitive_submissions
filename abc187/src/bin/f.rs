#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, m: usize);
    let mut g = vec![vec![0; n]; n];
    for _ in 0..m {
        input!(mut a: usize, mut b: usize);
        a -= 1; b -= 1;
        g[a][b] = 1;
        g[b][a] = 1;
    }
    let mut ans = 0;
    for mask in (0..1<<n).rev() {
        
    }
}
