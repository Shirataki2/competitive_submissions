#![allow(unused_imports)]
use proconio::{input, fastout};
use itertools_num::ItertoolsNum as _;
use std::cmp::*;

fn is_unlucky(v: u64) -> u64 {
    if v == 0 { return 0 }
    if format!("{}", v).contains("7") { return 0 }
    if format!("{:o}", v).contains("7") { return 0 }
    1
}

#[fastout]
fn main() {
    let v = (0..100_001).map(|v| is_unlucky(v)).cumsum().collect::<Vec<u64>>();
    input!(n: usize);
    println!("{}", v[n]);
}
