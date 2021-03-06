#![allow(unused_imports)]
use proconio::{input, fastout, marker::*};
use std::cmp::*;

#[fastout]
fn main() {
    input!(s: Bytes);
    println!("{}", if s[2] == s[3] && s[4] == s[5] { "Yes" } else { "No" });
}
