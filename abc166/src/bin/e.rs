#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;
use std::collections::*;

#[fastout]
fn main() {
    input!(n: usize, a: [i64; n]);
    let mut l = HashMap::<i64, i64>::new();
    let mut r = HashMap::<i64, i64>::new();
    a.iter().enumerate().for_each(|(i, &ai)| {
        *l.entry(i as i64 + 1 + ai).or_insert(0) += 1;
        *r.entry(i as i64 + 1 - ai).or_insert(0) += 1;
    });
    println!("{}", l.iter().fold(0, |acc, (k, &v)| {
        acc + v * r.get(k).unwrap_or(&0)
    }));
}
