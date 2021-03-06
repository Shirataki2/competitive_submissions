#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(k: u32, a: u32, b: u32);
    let mut f = false;
    for i in 1..1000 {
        f |= k * i >= a && k * i <= b;
    }
    println!("{}", if f { "OK" } else { "NG" });
}
