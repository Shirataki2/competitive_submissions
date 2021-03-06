#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, ab: [(u64, u64); n]);
    let mut a: Vec<_> = ab.iter().map(|(a, _b)| a).collect();
    let mut b: Vec<_> = ab.iter().map(|(_a, b)| b).collect();
    a.sort();
    b.sort();
    if n % 2 == 1 {
        let ma = a[(a.len()-1)/2];
        let mb = b[(b.len()-1)/2];
        println!("{}", mb - ma + 1);
    } else {
        let ma = a[a.len()/2-1] + a[a.len()/2];
        let mb = b[b.len()/2-1] + b[b.len()/2];
        println!("{}", mb - ma + 1);
    }
}
