#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;

fn main() {
    input!(n: usize, s: String);
    let s: Vec<_> = s.chars().collect();
    if n % 2 == 1 {
        println!("No");
        return;
    }

    let (s, t) = s.split_at(n / 2);
    println!(
        "{}",
        if s.iter().zip(t.iter()).all(|(s, t)| s == t) {
            "Yes"
        } else {
            "No"
        }
    );
}
