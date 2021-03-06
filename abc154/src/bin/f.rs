#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;

use procon_bicoef::ModBicoef;
use procon_modint::{set_modint, DynamicModInt};

/// y = kのとき
/// T(k) = C(c1+k, k) + C(c1+1+k, k) + ... + C(c2+k, k) を求めればよい
/// S(k) = \sum_{i=0}^{N} C(i+k, k)
///      = C(N+k+1, k + 1)
/// T(k) = S(c2) - S(c1-1)
/// が成立(矩形上の経路を考察する)する
fn main() {
    set_modint(1_000_000_007i64);
    let c = ModBicoef::small_new(2_000_001);
    input!(c1: usize, r1: usize, c2: usize, r2: usize);
    let mut ans = DynamicModInt::new(0);
    for y in r1..=r2 {
        ans += c.comb(c2 + y + 1, y + 1) - c.comb(c1 + y, y + 1);
    }
    println!("{}", ans.value());
}
