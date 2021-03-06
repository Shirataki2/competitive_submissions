#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    let mut is_sq = vec![false; 250001];
    for i in 1..=500 {
        is_sq[i*i] = true;
    }
    input!(n: usize, d: usize, x: [[i64; d]; n]);
    let mut ans = 0;
    for i in 0..n-1 {
        for j in i+1..n {
            let k = x[i].iter().zip(x[j].clone()).fold(0, |acc, (xi, xj)| acc + (xj - xi) * (xj - xi));
            if is_sq[k as usize] {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
