#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(a: i64, b: i64, _c: i64, k: i64);
    if a >= k {
        println!("{}", k);
    } else {
        println!("{}", a - (k - a - b));
    }
}
