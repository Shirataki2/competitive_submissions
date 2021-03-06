#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;

use procon_itertools::LexicalPermutation;

fn d(p: &[usize], coords: &[(f64, f64)]) -> f64 {
    let dist = |a: (f64, f64), b: (f64, f64)| (a.0 - b.0).hypot(a.1 - b.1);
    let mut ret = 0.0;
    for i in 0..p.len() - 1 {
        let cur = p[i];
        let nxt = p[i+1];
        ret += dist(coords[cur], coords[nxt]);
    }
    ret
}

fn main() {
    input!(n: usize, coords: [(f64, f64); n]);
    let mut p = (0..n).collect::<Vec<_>>();
    let mut ans = d(&p, &coords);
    while p.next_permutation() {
        ans += d(&p, &coords);
    }
    ans /= (1..=n).fold(1.0, |acc, v| acc * v as f64);
    println!("{}", ans);
}

#[macro_export]
macro_rules! min {
    ($a: expr) => { $a };
    ($a: expr, $b: expr) => { std::cmp::min($a, $b) };
    ($a: expr, $($rest: expr),+) => { std::cmp::min($a, min!($($rest),+)) };
}

#[macro_export]
macro_rules! chmin {
    ($a: expr, $($rest: expr),+) => {{
        let cmp_min = min!($($rest),+);
        if $a > cmp_min { $a = cmp_min; true } else { false }
    }};
}
