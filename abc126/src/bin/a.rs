#![allow(unused_imports)]
use proconio::{input, fastout, marker::*};
use std::cmp::*;

#[fastout]
fn main() {
    input!(_n: usize, k: Usize1, s: Chars);
    println!("{}",
        s
        .iter()
        .enumerate()
        .map(|(i, &c)| if i == k { c.to_ascii_lowercase() } else { c })
        .collect::<String>()
    );
}
