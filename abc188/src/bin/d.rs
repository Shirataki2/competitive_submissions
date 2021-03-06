#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, c: i64);
    let mut prices = vec![(0, 0); 2*n];
    for i in 0..n {
        input!(a: i64, b: i64, cc: i64);
        prices[2*i] = (a, cc);
        prices[2*i+1] = (b+1, -cc);
    }
    prices.sort();
    let mut ans = 0;
    let mut ppd = prices[0].1;
    for i in 0..2*n-1 {
        let dur = prices[i+1].0 - prices[i].0;
        if ppd > c {
            ans += c * dur;
        } else {
            ans += ppd * dur;
        }
        ppd += prices[i+1].1;
    }
    println!("{}", ans);
}
