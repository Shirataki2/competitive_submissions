use proconio::{input, fastout};
#[allow(unused_imports)]
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, d: i64);
    let d = d * d;
    let mut ans = 0;
    for _ in 0..n {
        input!(x: i64, y: i64);
        if x * x + y * y <= d {
            ans += 1;
        }
    }
    println!("{}", ans);
}
