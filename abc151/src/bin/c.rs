#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;

fn main() {
    input!(n: usize, m: usize);
    let mut wa_ctr = vec![0; n];
    let mut ac_sum = 0;
    let mut wa_sum = 0;
    let mut accepted = vec![false; n];
    for _ in 0..m {
        input!(p: Usize1, s: String);
        match s.as_str() {
            "AC" => {
                if !accepted[p] {
                    accepted[p] = true;
                    wa_sum += wa_ctr[p];
                    ac_sum += 1;
                }
            },
            "WA" => {
                if !accepted[p] {
                    wa_ctr[p] += 1;
                }
            },
            _ => unreachable!(""),
        }
    }
    println!("{} {}", ac_sum, wa_sum);
}
