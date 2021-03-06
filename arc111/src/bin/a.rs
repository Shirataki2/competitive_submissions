#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: i64, m: i64);
    let r = powmod(10, n, m * m) / m % m ;
    println!("{}", r);
}

pub fn signed_mod(a: i64, m: i64) -> i64 {
    (a % m + m) % m
}

pub fn invmod(a: i64, m: i64) -> i64 {
    let (_d, x, _y) = extgcd(a, m);
    signed_mod(x, m)
}

pub fn powmod(mut x: i64, mut n: i64, modulo: i64) -> i64 {
    let mut ret = 1;
    while n > 0 {
        if n & 1 > 0 {
            ret = (ret * x) % modulo;
        }
        x = (x * x) % modulo;
        n >>= 1;
    }
    ret
}

pub mod modbicoef {
    type Num = i64;

    pub struct Combination {
        m: Num,
        fac: Vec<Num>,
        ifac: Vec<Num>,
    }

    impl Combination {
        pub fn new(n: usize, m: Num) -> Self {
            let mut fac = vec![0; n];
            let mut inv = vec![0; n];
            let mut ifac = vec![0; n];
            fac[0] = 1;
            fac[1] = 1;
            ifac[0] = 1;
            ifac[1] = 1;
            inv[1] = 1;
            for i in 2..n {
                let iu = i as i64;
                fac[i] = fac[i - 1] * iu % m;
                inv[i] = m - inv[m as usize % i] * (m / iu) % m;
                ifac[i] = ifac[i - 1] * inv[i] % m;
            }
            Self { m, fac, ifac }
        }

        pub fn comb(&self, n: usize, r: usize) -> Num {
            let m = self.m;
            if n < r {
                0
            }
            else {
                self.fac[n] * (self.ifac[r] * self.ifac[n - r] % m) % m
            }
        }
    }
}

pub fn gcd(a: i64, b: i64) -> i64 {
    match (a, b) {
        (a, 0) => a,
        (a, b) => gcd(b, a % b)
    }
}

pub fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

pub fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b > 0 {
        let (gcd, mut y, x) = extgcd(b, a % b);
        y -= (a / b) * x;
        (gcd, x, y)
    } else {
        (a, 1, 0)
    }
}