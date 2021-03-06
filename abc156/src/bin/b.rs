#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(mut n: i64, k: i64);
    let mut ans = 0;
    while n > 0 {
        ans += 1;
        n /= k;
    }
    println!("{}", ans);
}
