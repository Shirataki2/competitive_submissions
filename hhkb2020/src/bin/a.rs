#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(s: char, t: char);
    let ans = match s {
        'Y' => t.to_ascii_uppercase(),
        _ => t
    };
    println!("{}", ans);
}
