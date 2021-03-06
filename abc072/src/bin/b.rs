#![allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
use std::cmp::*;

#[fastout]
fn main() {
    input!(s: Chars);
    println!("{}", s.iter().step_by(2).collect::<String>());
}
