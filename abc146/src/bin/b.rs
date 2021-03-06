#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

fn rot(s: &str, n: usize) -> String {
    s.chars().map(|c| {
        let ord = c as usize - 'A' as usize;
        let ord = (ord + n) % 26;
        (ord as u8 + 'A' as u8) as char
    }).collect()
}

#[fastout]
fn main() {
    input!(n: usize, s: String);
    println!("{}", rot(&s, n));
}
