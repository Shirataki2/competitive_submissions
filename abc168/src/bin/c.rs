#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(a: f64, b: f64, h: f64, m: f64);
    let deg_h = 30.0 * h + 30.0 / 60.0 * m;
    let deg_m = 360.0 / 60.0 * m;
    let rad_diff = (deg_m - deg_h) / 180.0 * std::f64::consts::PI;
    let ans = (a * a + b * b - 2.0 * a * b * rad_diff.cos()).sqrt();
    println!("{}", ans);
}
