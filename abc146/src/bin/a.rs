#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;
use std::collections::*;
#[fastout]
fn main() {
    let mut map = HashMap::new();
    for (i, &s) in vec!["SAT", "FRI", "THU", "WED", "TUE", "MON", "SUN"].iter().enumerate() {
        map.insert(s, i+1);
    }
    input!(s: String);
    println!("{}", map[s.as_str()]);
}
