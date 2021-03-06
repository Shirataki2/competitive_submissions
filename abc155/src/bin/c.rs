#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;
use std::collections::*;

#[fastout]
fn main() {
    input!(n: usize, s: [String; n]);
    let mut map = BTreeMap::new();
    s.iter().for_each(|s| {
        *map.entry(s).or_insert(0) += 1;
    });
    let max_voted_count = map.iter().fold(0, |acc, (_k, &v)| {
        max(acc, v)
    });
    for (k, v) in map {
        if v == max_voted_count {
            println!("{}", k);
        }
    }
}
