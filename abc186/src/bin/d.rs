#![allow(unused_imports)]
use proconio::{input, fastout};
use itertools_num::ItertoolsNum as _;
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, mut a: [i64; n]);
    a.sort();
    let ca = a.iter().cumsum().collect::<Vec<i64>>();
    let mut ans = 0;
    for i in 0..n-1 {
        ans += (ca[n-1]-ca[i]) - ((n - i - 1) as i64) * a[i];
    }
    println!("{}", ans);
}
