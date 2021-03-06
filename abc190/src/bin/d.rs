#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: i64);
    let ds = divisor(2*n);
    let mut ans = 0;
    for d in ds.iter() {
        if (2 * n / d - d).rem_euclid(2) == 1 { ans += 1; } 
    }
    println!("{}", ans);
}

pub fn divisor(n: i64) -> Vec<i64> {
    let mut ret = Vec::new();
    for i in 1.. {
        if i * i > n { break }
        if n % i == 0 {
            ret.push(i);
            if i * i != n { ret.push(n / i); }
        }
    }
    ret.sort_unstable();
    ret
}
