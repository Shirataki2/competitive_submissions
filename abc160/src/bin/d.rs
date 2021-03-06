#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: i64, mut x: i64, mut y: i64);
    x -= 1; y -= 1;
    let mut ans = vec![0; n as usize];
    for i in 0..n-1 {
        for j in i+1..n {
            //   --2-3-4- 
            //   |      |
            // 0-1------5-6
            // 1) j - i(経由しない)
            // 2) |x - i| + |y - j| + 1
            // 3) |y - i| + |x - j| + 1
            let d = min!(j - i, (x - i).abs() + (y - j).abs() + 1, (y - i).abs() + (x - j).abs() + 1);
            ans[d as usize] += 1;
        }
    }
    for i in 1..n {
        println!("{}", ans[i as usize]);
    }
}

#[macro_export]
macro_rules! min {
    ($a: expr) => { $a };
    ($a: expr, $b: expr) => { std::cmp::min($a, $b) };
    ($a: expr, $($rest: expr),+) => { std::cmp::min($a, min!($($rest),+)) };
}
