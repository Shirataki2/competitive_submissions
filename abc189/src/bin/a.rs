#![allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
use std::cmp::*;

#[fastout]
fn main() {
    input!(s: Chars);
    println!("{}", if s[0] == s[1] && s[1] == s[2] { "Won" } else { "Lost" });
}
