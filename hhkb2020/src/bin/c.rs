#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize);
    let mut minima = 0;
    let mut ctr = vec![0; 200_001];
    for _ in 0..n {
        input!(p: usize);
        ctr[p] += 1;
        while ctr[minima] > 0 {
            minima += 1;
        }
        println!("{}", minima);
    }
}
