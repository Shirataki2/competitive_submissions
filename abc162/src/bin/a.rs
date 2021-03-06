#![allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: Chars);
    println!("{}", if n.iter().any(|&c| c == '7') { "Yes" } else { "No" });
}
