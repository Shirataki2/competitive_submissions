#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(s: String);
    println!("{}", if &s == "ARC" { "ABC" } else { "ARC" });
}
