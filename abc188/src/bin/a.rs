#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(mut a: [i64; 2]);
    a.sort();
    println!("{}", if a[0] + 3 > a[1] { "Yes" } else { "No" });
}
