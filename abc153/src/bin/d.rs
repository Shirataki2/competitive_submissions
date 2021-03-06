#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(mut h: i64);
    let mut ans = 0;
    let mut m: i64 = 1;
    while h > 0 {
        ans += m;
        m *= 2;
        h /= 2;
    }
    println!("{}", ans);
}
