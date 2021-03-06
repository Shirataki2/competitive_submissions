#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(s: String, t: String);
    println!("{}", if t.starts_with(&s) { "Yes" } else { "No" });
}
