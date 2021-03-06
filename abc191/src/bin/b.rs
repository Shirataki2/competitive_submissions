#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;
use std::collections::*;

#[fastout]
fn main() {
    input!(n: usize, x: i64, a: [i64; n]);
    let a = a.iter().filter(|&ai| *ai != x).collect::<Vec<_>>();
    println!("{}", a.iter().map(|a| format!("{}", a)).collect::<Vec<_>>().join(" "));
}
