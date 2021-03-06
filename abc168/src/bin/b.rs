#![allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
use std::cmp::*;

#[fastout]
fn main() {
    input!(k: usize, s: Chars);
    if k >= s.len() {
        println!("{}", s.into_iter().collect::<String>());
    } else {
        println!("{}...", s[..k].into_iter().collect::<String>());
    }
}
