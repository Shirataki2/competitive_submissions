#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(x: i64, n: usize, p: [i64; n]);
    for i in 0..100 {
        if !p.contains(&(x - i)) {
            println!("{}", x - i);
            return;
        }
        if !p.contains(&(x + i)) {
            println!("{}", x + i);
            return;
        }
    }
}
