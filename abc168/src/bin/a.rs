#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: u16);
    println!("{}", match n % 10 {
        3             => "bon",
        0 | 1 | 6 | 8 => "pon",
        _             => "hon",
    });
}
