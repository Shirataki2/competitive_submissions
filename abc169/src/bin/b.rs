#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

const MAX: i128 = 1_000_000_000_000_000_000;

#[fastout]
fn main() {
    input!(n: usize, a: [i128; n]);
    let mut ans = 1;
    for ai in a.iter() {
        if *ai == 0 {
            println!("0");
            return
        }
    }
    for ai in a.iter() {
        ans *= ai;
        if ans > MAX {
            println!("-1");
            return
        }
    }
    println!("{}", ans);
}
