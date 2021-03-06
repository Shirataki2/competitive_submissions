#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;

use procon_ntt::Ntt998244353;

fn main() {
    input!(n: usize, m: usize, a: [i64; n], b: [i64; m]);
    let a: Ntt998244353 = a.into();
    let b: Ntt998244353 = b.into();
    let c = a * b;
    print!("{}", c[0]);
    for c in c[1..=n+m-2].iter() {
        print!(" {}", c);
    }
    println!();
}
