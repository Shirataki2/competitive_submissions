#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;
use std::collections::{HashMap, BTreeSet};

#[fastout]
fn main() {
    const A_MAX: u32 = 1_000_001;
    input!(n: usize, mut a: [u32; n]);
    a.sort();
    let mut ctr = HashMap::new();
    let mut f = vec![true; A_MAX as usize];
    a.iter().for_each(|&ai| *ctr.entry(ai).or_insert(0) += 1);
    let a: BTreeSet<_> = a.drain(..).collect();
    a.iter().for_each(|&ai| {
        if ctr[&ai] >= 2 {
            f[ai as usize] = false;
        }
        let mut i = 2 * ai;
        while i < A_MAX {
            f[i as usize] = false;
            i += ai;
        }
    });
    println!("{}", a.iter().filter(|&ai| f[*ai as usize]).count());
}
