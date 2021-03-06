#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(s: String);
    let mut prefix = Vec::new();
    let mut suffix = Vec::new();
    let mut is_rev = false;
    input!(q: usize);
    for _ in 0..q {
        input!(t: usize);
        match t {
            1 => is_rev = !is_rev,
            2 => {
                input!(f: usize, c: char);
                match (f, is_rev) {
                    (1, false) | (2, true) => { prefix.push(c) },
                    (1, true) | (2, false) => { suffix.push(c) },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }
    if !is_rev {
        for c in prefix.iter().rev() { print!("{}", c); }
        print!("{}", s);
        for c in suffix.iter() { print!("{}", c); }    
    } else {
        for c in suffix.iter().rev() { print!("{}", c); }    
        for c in s.chars().rev() { print!("{}", c); }
        for c in prefix.iter() { print!("{}", c); }
    }
    println!();
}
