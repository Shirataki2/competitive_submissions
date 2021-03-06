#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(x: i64);
    for a in -500..=500 {
        for b in -500..=500 {
            if a*a*a*a*a-b*b*b*b*b == x {
                println!("{} {}", a, b);
                return
            }
        }
    }
}
