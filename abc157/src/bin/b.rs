#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;
use std::collections::*;

#[fastout]
fn main() {
    input!(a: [i64; 9], n: usize, b: [i64; n]);
    let inv_a = a
        .iter()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect::<HashMap<_, _>>();
    let mut map = vec![false; 9];    
    for b in b.iter() {
        if let Some(&num) = inv_a.get(b) {
            map[num] = true;
        }
    }
    let ans = 
        (map[0] && map[3] && map[6]) ||
        (map[1] && map[4] && map[7]) ||
        (map[2] && map[5] && map[8]) ||
        (map[0] && map[1] && map[2]) ||
        (map[3] && map[4] && map[5]) ||
        (map[6] && map[7] && map[8]) ||
        (map[0] && map[4] && map[8]) ||
        (map[2] && map[4] && map[6]);
    println!("{}", if ans { "Yes" } else { "No" });
}
