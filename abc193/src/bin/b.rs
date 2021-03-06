#![allow(unused_imports)]
use proconio::{input, fastout, marker::*};
use std::{cmp::*, collections::*};

#[fastout]
fn main() {
    input!(n: usize);
    let mut ans = 1i64 << 60;
    for _ in 0..n {
        input!(a: i64, p: i64, x: i64);
        if x - a > 0 {
            ans = min(ans, p);
        }
    }
    println!("{}", if ans == 1i64 << 60 { -1 } else { ans })
}
