#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(a: f64, b: f64);
    for n in 0..10100 {
        if (n as f64 * 0.08).floor() == a && 
           (n as f64 * 0.10).floor() == b
        {
            println!("{}", n as u64);
            return;
        }
    }
    println!("{}", -1);
}
