#![allow(unused_imports, dead_code)]
use proconio::{input, fastout};
use std::cmp::*;

pub fn gcd(a: i64, b: i64) -> i64 {
    match (a, b) {
        (a, 0) => a,
        (a, b) => gcd(b, a % b),
    }
}

pub fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b > 0 {
        let (d, mut y, x) = extgcd(b, a % b);
        y -= (a / b) * x;
        (d, x, y)
    } else {
        (a, 1, 0)
    }
}
pub mod modulo {
    use super::extgcd;
    use std::cell::RefCell;
    type Num = i64;
    thread_local ! (static MOD : RefCell < Num > = RefCell :: new (0 ) );
    pub fn set_mod(m: Num) {
        MOD.with(|x| x.replace(m));
    }
    pub fn modulo() -> Num {
        MOD.with(|x| *x.borrow())
    }
    pub fn signed_mod(a: Num) -> Num {
        let m = modulo();
        (a % m + m) % m
    }
    /// Modulo and `a` must be coprime integers.
    pub fn invmod(a: Num) -> Num {
        let m = modulo();
        let (_d, x, _y) = extgcd(a, m);
        signed_mod(x)
    }
    /// Modulo must be a prime number.
    pub struct Comb {
        n: usize,
        fac: Vec<Num>,
        inv: Vec<Num>,
        ifac: Vec<Num>,
    }
    impl Comb {
        pub fn new(n: usize) -> Self {
            let mut fac = vec![0; n];
            let mut inv = vec![0; n];
            let mut ifac = vec![0; n];
            fac[0] = 1;
            fac[1] = 1;
            ifac[0] = 1;
            ifac[1] = 1;
            inv[1] = 1;
            let m = modulo();
            for i in 2..n {
                let iu = i as i64;
                fac[i] = fac[i - 1] * iu % m;
                inv[i] = m - inv[m as usize % i] * (m / iu) % m;
                ifac[i] = ifac[i - 1] * inv[i] % m;
            }
            Self { n, fac, inv, ifac }
        }
        pub fn comb(&self, n: usize, r: usize) -> Num {
            let m = modulo();
            if n < r {
                0
            } else {
                self.fac[n] * (self.ifac[r] * self.ifac[n - r] % m) % m
            }
        }
    }
}

#[fastout]
fn main() {
    use modulo::*;

    input!(t: usize);
    for _ in 0..t {
        input!(n: i64, s: i64, k: i64);
        let d = gcd(k, n);
        let ans = match s % d {
            0 => {
                let (n, s, k) = (n / d, s / d, k / d);
                set_mod(n);
                (n - s) * invmod(k) % n
            },
            _ => -1
        };
        println!("{}", ans);
    }
}
