#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(x: i32, y: i32);
    let mut ans = false;
    for tsuru in 0..=x {
        let kame = x - tsuru;
        if 2 * tsuru + 4 * kame == y {
            ans = true;
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
