#![allow(unused_imports)]
use proconio::{input, fastout, marker::*};
use std::{cmp::*, collections::*};

#[fastout]
fn main() {
    input!(a: f64, b: f64);
    println!("{}", 100.0 * (a - b) / a);
}
