#![allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
use std::cmp::*;

fn g1(s: &Vec<char>) -> i64 {
    let mut s = s.clone();
    s.sort();
    s.reverse();
    let s = s.iter().collect::<String>();
    s.parse::<i64>().unwrap()
}

fn g2(s: &Vec<char>) -> i64 {
    let mut s = s.clone();
    s.sort();
    let s = s.iter().collect::<String>();
    s.parse::<i64>().unwrap()
}

fn f(s: &Vec<char>) -> Vec<char> {
    let a1 = g1(s);
    let a2 = g2(s);
    (a1 - a2).to_string().chars().collect()
}

#[fastout]
fn main() {
    input!(mut n: Chars, k: usize);
    for _ in 0..k {
        n = f(&n);
    }
    println!("{}", n.iter().collect::<String>());
}
