#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

fn dfs(x: i64, a: &mut Vec<i64>) {
    a.push(x);
    if x > 4_000_000_000 { return }
    for dx in -1..=1 {
        if x % 10 + dx >= 0 && x % 10 + dx <= 9 {
            dfs(x * 10 + x % 10 + dx, a);
        }
    }
} 

#[fastout]
fn main() {
    let mut a = vec![];
    for i in 1..=9 {
        dfs(i, &mut a);
    }
    a.sort();
    input!(n: usize);
    println!("{}", a[n-1]);
}
