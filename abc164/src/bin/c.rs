#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;
use std::collections::*;

#[fastout]
fn main() {
    input!(n: usize);
    let mut map = HashSet::new();
    for _ in 0..n {
        input!(s: String);
        map.insert(s);
    }
    println!("{}", map.len());
}
