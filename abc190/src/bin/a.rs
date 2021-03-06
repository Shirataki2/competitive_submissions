#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

fn main() {
    input!(mut a: i64, mut b: i64, mut c: i64);

    loop {
        if c % 2 == 0 {
            if a == 0 {
                println!("Aoki");
                return;
            }
            a -= 1;
        } else {
            if b == 0 {
                println!("Takahashi");
                return;
            }
            b -= 1;
        }
        c += 1;
    }
}
