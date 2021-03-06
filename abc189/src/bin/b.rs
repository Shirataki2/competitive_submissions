#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, x: i64);
    let mut a = 0;
    for i in 0..n {
        input!(v: i64, p: i64);
        a += v * p;
        if x * 100 < a {
            println!("{}", i + 1);
            return
        }
    }
    println!("{}", -1);
}
