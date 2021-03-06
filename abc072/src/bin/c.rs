#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, a: [usize; n]);
    let mut ctr = vec![0; 100_002];
    for a in a.iter() { ctr[*a] += 1; }
    let mut ans = 0;
    for i in 1..=100000 {
        let c = ctr[i-1] + ctr[i] + ctr[i+1];
        if ans < c {
            ans = c;
        }
    }
    println!("{}", ans);
}
