#![allow(unused_imports)]
use proconio::{input, fastout, marker::Usize1};
use std::cmp::*;

#[fastout]
fn main() {
    //! 1 2 -> 2 1 (xx -> oo)
    //! 3 2 -> 2 3 (ox -> xo)
    //! 1 4 -> 4 1 (xo -> oo)
    input!(n: usize, mut a: [Usize1; n]);
    let mut ans = 0;
    for i in 0..n-1 {
        if a[i] == i {
            a.swap(i, i+1);
            ans += 1;
        }
    }
    if a[n-1] == n - 1 { ans += 1; }
    println!("{}", ans);
}
