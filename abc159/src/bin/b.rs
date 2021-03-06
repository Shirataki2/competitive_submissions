#![allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
use std::cmp::*;

fn is_palindrome(s: &[char]) -> bool {
    let mut t = s.clone().to_vec();
    t.reverse();
    s.iter().zip(t.iter()).all(|(&s, &t)| s == t)
}

#[fastout]
fn main() {
    input!(s: Chars);
    let n = s.len();
    let ans = is_palindrome(&s) && is_palindrome(&s[..n/2]) && is_palindrome(&s[n/2+1..]);
    println!("{}", if ans { "Yes" } else { "No" });
}
