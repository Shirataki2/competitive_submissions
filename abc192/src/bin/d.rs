#![allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
use std::cmp::*;

fn check(n: u128, x: &[char], m: u128) -> bool {
    // オーバーフロー怖いのでmをn進にして桁数で比較する
    use Ordering::*;

    let mut y = vec![];
    let mut m1 = m;
    while m1 > 0 {
        y.push(m1 % n);
        m1 /= n;
    }
    y.reverse();
    match x.len().cmp(&y.len()) {
        Greater => return false,
        Less => return true,
        Equal => {}
    };

    // 桁が同じ
    for (&xi, &yi) in x.iter().zip(y.iter()) {
        let xi = xi.to_digit(10).unwrap() as u128;
        match xi.cmp(&yi) {
            Greater => return false,
            Less => return true,
            Equal => {}
        };
    }
    true
}

#[fastout]
fn main() {
    input!(x: Chars, m: u128);
    if x.len() == 1 {
        let x: u128 = x.iter().collect::<String>().parse().unwrap();
        // 「見なして得られる値」
        println!("{}", if x <= m { "1" } else { "0" });
    } else {
        let max_digit = x.iter().map(|x| x.to_digit(10).unwrap() as u128).max().unwrap();
        let (mut l, mut r) = (max_digit, 10_000_000_000_000_000_001);
        while l + 1 < r {
            let mid = (l + r) / 2;
            if check(mid, &x, m) {
                l = mid;
            } else {
                r = mid;
            }
        }
        println!("{}", l - max_digit);
    }
}
