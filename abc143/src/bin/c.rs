#![allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, s: Chars);
    let mut ans = 1;
    for i in 0..n-1 {
        if s[i] != s[i+1] { ans += 1; }
    }
    println!("{}", ans);
}
