#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
use std::cmp::*;
use std::collections::*;
#[macro_use]
extern crate procon_input as _;

fn main() {
    input!(n: usize, k: i32, m: i32, a: [i32; n-1]);
    let diff = n as i32 * m - a.iter().sum::<i32>();
    if diff > k {
        println!("{}", -1);
    } else {
        println!("{}", max(0, diff));
    }
}
