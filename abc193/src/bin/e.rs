#![allow(unused_imports)]
use proconio::{input, fastout, marker::*};
use std::{cmp::*, collections::*};

#[fastout]
fn main() {
    input!(n: usize);
    for _ in 0..n {
        let mut ans = 1i128 << 126;
        input!(x: i128, y: i128, p: i128, q: i128);
        let modulii = [2 * x + 2 * y, p + q];
        for r1 in x..x+y {
            for r2 in p..p+q {
                let residues = [r1, r2];
                if let Some(s) = crt(&residues, &modulii) {
                    chmin!(ans, s);
                }
            }
        }
        if ans == 1i128 << 126 {
            println!("infinity");
        } else {
            println!("{}", ans);
        }
    }
}

fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
 
fn mod_inv(x: i128, n: i128) -> Option<i128> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
 
fn crt(residues: &[i128], modulii: &[i128]) -> Option<i128> {
    let prod = modulii.iter().product::<i128>();
 
    let mut sum = 0;
 
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
 
    Some(sum % prod)
}

#[macro_export]
macro_rules! min {
    ($a: expr) => { $a };
    ($a: expr, $b: expr) => { std::cmp::min($a, $b) };
    ($a: expr, $($rest: expr),+) => { std::cmp::min($a, min!($($rest),+)) };
}

#[macro_export]
macro_rules! chmin {
    ($a: expr, $($rest: expr),+) => {{
        let cmp_min = min!($($rest),+);
        if $a > cmp_min { $a = cmp_min; true } else { false }
    }};
}
