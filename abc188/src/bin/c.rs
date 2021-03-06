#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, a: [i64; 1<<n]);
    let mut a = a.iter().enumerate().collect::<Vec<_>>();
    for _ in 0..n-1 {
        for j in 0..a.len()>>1 {
            if a[2*j].1 > a[2*j+1].1 {
                a[j] = a[2*j];
            } else {
                a[j] = a[2*j+1];
            }
        }
        a.truncate(a.len()/2);
    }
    println!("{}", if a[0].1 < a[1].1 { a[0].0 + 1 } else { a[1].0 + 1 });
}
