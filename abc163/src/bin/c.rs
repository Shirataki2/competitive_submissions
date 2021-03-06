#![allow(unused_imports)]
use proconio::{input, fastout, marker::Usize1};
use std::cmp::*;
use std::collections::*;

#[fastout]
fn main() {
    input!(n: usize, a: [Usize1; n-1]);
    let mut ctr = vec![0; n];
    for ai in a.iter() {
        ctr[*ai] += 1;
    }
    for i in 0..n {
        println!("{}", ctr[i])
    }
}
