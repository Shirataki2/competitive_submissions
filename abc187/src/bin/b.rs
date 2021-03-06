#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, xy: [(i64, i64); n]);
    let mut ans = 0;
    for i in 0..n-1 {
        for j in i+1..n {
            let dx = xy[j].0 - xy[i].0;
            let dy = xy[j].1 - xy[i].1;
            if dx.abs() >= dy.abs() {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
