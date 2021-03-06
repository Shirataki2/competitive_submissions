use proconio::{input, fastout};
#[allow(unused_imports)]
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, mut a: [u64; n]);
    a.sort();
    a.reverse();
    let ans = (1..=n-1).fold(0, |acc, i| acc + a[i/2]);
    println!("{}", ans);
}
