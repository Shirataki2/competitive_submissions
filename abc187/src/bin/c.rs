#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;
use std::collections::*;

#[fastout]
fn main() {
    input!(n: usize, s: [String; n]);
    let mut s1 = HashSet::new();
    let mut s2 = HashSet::new();
    for i in 0..n {
        if s[i].starts_with('!') {
            s2.insert(s[i][1..].to_string());
        } else {
            s1.insert(s[i].to_string());
        }
    }
    for k in s1.iter() {
        if s2.contains(k) {
            println!("{}", k);
            return
        }
    }
    println!("satisfiable");
}
