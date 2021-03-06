#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, m: usize);
    let mut a = Vec::with_capacity(m);
    let mut b = Vec::with_capacity(m);
    for _ in 0..m {
        input!(x: usize, y: usize);
        a.push(x-1); b.push(y-1);
    }
    input!(k: usize);
    let mut c = Vec::with_capacity(k);
    let mut d = Vec::with_capacity(k);
    for _ in 0..k {
        input!(x: usize, y: usize);
        c.push(x-1); d.push(y-1);
    }
    let mut ans = 0;
    for i in 0..1<<k {
        let mut x = 0;
        let mut t = vec![0; n];
        for j in 0..k {
            t[if i >> j & 1 > 0 { c[j] } else { d[j] }] += 1;
        }
        for j in 0..m {
            x += if t[a[j]] > 0 && t[b[j]] > 0 { 1 } else { 0 };
        }
        ans = max(ans, x);
    }
    println!("{}", ans);
}
