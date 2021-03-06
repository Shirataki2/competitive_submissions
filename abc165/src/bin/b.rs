#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(x: u128);
    let mut c: u128 = 100;
    for i in 1.. {
        c = c * 101 / 100;
        if c >= x {
            println!("{}", i);
            return
        }
    }
}
