use proconio::{input, fastout};
#[allow(unused_imports)]
use std::cmp::*;

macro_rules! max {
    ($a: expr) => { $a };
    ($a: expr, $b: expr) => { std::cmp::max($a, $b) };
    ($a: expr, $($rest: expr), +) => { std::cmp::max($a, max!($($rest),+)) };
}
macro_rules! chmax {
    ($a: expr, $($rest: expr),+) => {{
        let cmp_max = max!($($rest),+);
        if $a < cmp_max { $a = cmp_max; true } else { false }
    }};
}

#[fastout]
fn main() {
    input!(n: usize, m: usize, k: i64);
    input!(a: [i64; n], b: [i64; m]);
    let mut cb = vec![0i64; m+1];
    for i in 0..m {
        cb[i+1] = cb[i] + b[i];
    }
    let mut ans = 0;
    let mut ca = 0;
    for i in 0..=n {
        if i != 0 { ca += a[i-1]; }
        if ca > k { break; }
        let mut l = 0;
        let mut r = m + 1;
        while r - l > 1 {
            let mid = (r + l) / 2;
            if ca + cb[mid] <= k {
                l = mid;
            } else {
                r = mid;
            }
        }
        chmax!(ans, i + l);
    }
    println!("{}", ans);
}
