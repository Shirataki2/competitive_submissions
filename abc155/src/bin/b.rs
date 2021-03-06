#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, a: [i64; n]);
    println!("{}", 
        if a.iter().all(|&v| {
            v % 2 == 1 || v % 3 == 0 || v % 5 == 0
        }) {
            "APPROVED"
        } else {
            "DENIED"
        }
    );
}
