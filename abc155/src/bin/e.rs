#![allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
use std::cmp::*;

#[fastout]
fn main() {
    input!(s: Chars);
    let n = s.len();
    // 桁上がり
    let mut up = false;
    let mut ans = 0;
    for i in (0..n).rev() {
        let c = s[i];
        let mut d: u32 = c.to_digit(10).unwrap();
        if up { d += 1; up = false; }
        // 下位の数からみて
        // 4 以下ならそのまま払って6 以上なら一桁上の紙幣を使う
        // 5 のときは　15 .. 10x1 + 1x5 = 6
        //           95 .. 100x1 + 1x5 = 6
        // 上の桁が~4,6~で判定する
        // 問題は上の桁が5のときだが
        // 55 .. 10x5 + 1x5 = 10
        // 55 .. 100x1 + 10x4 + 1x5 = 10
        // 使う枚数は一緒になるので好きな方に分類すれば良さそう
        match d {
            0..=4 => ans += d,
            5 => {
                if i > 0 && s[i-1].to_digit(10).unwrap() >= 5 {
                    ans += 10 - d;
                    up = true;
                } else {
                    ans += d;
                }
            },
            6..=10 => {
                ans += 10 - d;
                up = true;
            },
            _ => unreachable!(),
        }
    }
    // 最上位で繰り上がった場合を忘れない(入力例2)
    if up { ans += 1; }
    println!("{}", ans);
}
