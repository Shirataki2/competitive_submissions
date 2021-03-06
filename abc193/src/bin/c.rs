#![allow(unused_imports)]
use proconio::{input, fastout, marker::*};
use std::{cmp::*, collections::*};

#[fastout]
fn main() {
    input!(n: i128);
    let mut set = HashSet::new();
    for a in 2..=100_000i128 {
        for b in 2.. {
            if a.pow(b) > n { break; }
            set.insert(a.pow(b));
        }
    }
    println!("{}", n - set.len() as i128)
}
