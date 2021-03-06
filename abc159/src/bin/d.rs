#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;
use std::collections::*;


#[fastout]
fn main() {
    input!(n: usize, a: [i64; n]);
    let mut ctr: HashMap<&i64, i64> = HashMap::new();
    for a in a.iter() { *ctr.entry(a).or_insert(0) += 1; }
    let s = ctr.iter().fold(0i64, |acc, (_, x)| acc + x * (x - 1) / 2);
    for a in a.iter() {
        let x = ctr.get(a).unwrap();
        println!("{}", s - x * (x - 1) / 2 + (x - 1) * (x - 2) / 2);
    }
}
