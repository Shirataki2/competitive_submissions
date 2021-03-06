#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;
use std::collections::*;

/// Naive prime factorization (O(N^(1/2)))
///
/// Returns a map of { prime: exponent }.
pub fn factorize(n: u64) -> HashMap<u64, u64> {
    let mut ret = HashMap::new();
    let mut n = n;
    while n % 2 == 0 {
        *ret.entry(2).or_insert(0) += 1;
        n /= 2;
    }
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            *ret.entry(i).or_insert(0) += 1;
            n /= i;
        }
        i += 2;
    }
    if n > 2 {
        *ret.entry(n).or_insert(0) += 1;
    }
    ret
}


#[fastout]
fn main() {
    input!(n: u64);
    let map = factorize(n);
    let ans = map.iter().fold(0, |acc, (_, m)| {
        let mut a = *m;
        let mut ctr = 0;
        for i in 1.. {
            if a >= i {
                ctr += 1;
                a -= i;
            } else {
                break
            }
        }
        acc + ctr
    });
    println!("{}", ans);
}
