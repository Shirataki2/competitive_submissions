#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(k: i64, x: i64);
    let ans = (x-k+1..x+k).collect::<Vec<_>>();
    println!("{}", ans.iter().map(|v| format!("{}", v)).collect::<Vec<_>>().join(" "));
}
