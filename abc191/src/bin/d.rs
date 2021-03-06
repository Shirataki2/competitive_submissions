#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

fn to_integer(s: &str) -> i64 {
    let v = s.split('.').collect::<Vec<_>>();
    match v.len() {
        1 => v[0].parse::<i64>().unwrap() * 10000,
        2 => {
            let a = v[0].parse::<i64>().unwrap() * 10000;
            let mut b = v[1].to_owned();
            while b.len() < 4 {
                b.push('0');
            }
            let b = b.parse::<i64>().unwrap();
            a + b
        },
        _ => unreachable!(),
    }
}

#[fastout]
fn main() {
    input!(x: String, y: String, r: String);
    let x = to_integer(&x);
    let y = to_integer(&y);
    let r = to_integer(&r);
    // 直線 x = cを左から走査的に見ていく
    // 10^4倍したので円と直線の交点の内部の線分に対して
    // y座標が10^4の倍数であるような点を数えていく

    let s = (x - r + 9999) / 10000 * 10000;
    let e = (x + r) / 10000 * 10000;

    let mut ans: i64 = 0;
    (s..).step_by(10000).take_while(|&x| x <= e).for_each(|c| {
        let dy = (r * r - (x - c) * (x - c)) as f64;
        let dy = dy.sqrt() as i64;
        let y1 = (y + dy) / 10000;
        let y2 = (y - dy + 9999) / 10000;
        ans += y1 - y2 + 1;
    });
    println!("{}", ans);
}

#[test]
fn test_to_integer() {
    assert_eq!(to_integer("1.2345"), 12345);
    assert_eq!(to_integer("1.2"), 12000);
    assert_eq!(to_integer("1"), 10000);
}