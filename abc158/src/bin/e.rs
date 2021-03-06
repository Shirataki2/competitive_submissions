#![allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
use std::cmp::*;
use std::collections::*;

#[fastout]
fn main() {
    input!(n: usize, p: usize);
    input!(mut s: Chars);
    let ans = match p {
        2 | 5 => {
            // 右一桁で判別する
            let mut ans = 0;
            for i in 0..n {
                if s[n-i-1].to_digit(10).unwrap() as usize % p == 0 {
                    ans += n - i;
                }
            }
            ans
        },
        _ => {
            // 10と互いに素でない素数である場合
            s.push('0');
            s.reverse();
            let mut map = HashMap::new();
            let mut b = 1;
            let mut d = 0;
            for s in s.iter() {
                let si = s.to_digit(10).unwrap() as usize;
                d = (d + si * b) % p;
                b = (b * 10) % p;
                *map.entry(d).or_insert(0) += 1usize;
            }
            map.iter().fold(0, |acc, (_, v)| acc + v * (v - 1) / 2)
        },
    };
    println!("{}", ans);
}
