#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(mut a: i64, b: i64, mut c: i64, d: i64);
    let mut ctr = 0;
    while a > 0 || c > 0 {
        if ctr % 2 != 0 {
            a -= d;
        } else {
            c -= b;
        }
        ctr += 1;
    }
    println!("{}", if ctr % 2 == 0 { "Yes" } else { "No" });
}
