#![allow(unused_imports)]
use proconio::{input, fastout, marker::*};
use std::{cmp::*, collections::*, ops::AddAssign};

fn point(d: &[char], d5: i64) -> i64 {
    let mut d = d.iter().map(|&c| c.to_digit(10).unwrap_or(0) as i64).collect::<Vec<_>>();
    d[4] = d5;
    let mut ctr = HashMap::new();
    for d in 1..=9 {
        ctr.entry(d).or_insert(0);
    }
    for i in 0..5 {
        ctr.entry(d[i] as i64).or_insert(0).add_assign(1);
    }
    ctr.iter().fold(0i64, |acc, (&i, &c)| {
        acc + i * 10i64.pow(c)
    })
} 

#[fastout]
fn main() {
    input!(k: usize, s: Chars, t: Chars);
    let mut deck = HashMap::new();
    for i in 1..=9 {
        deck.entry(i).or_insert(k as i64);
    }
    for &c in s.iter() {
        if c == '#' { break; }
        let d = c.to_digit(10).unwrap() as i64;
        *deck.entry(d).or_default() -= 1;
    }
    for &c in t.iter() {
        if c == '#' { break; }
        let d = c.to_digit(10).unwrap() as i64;
        *deck.entry(d).or_default() -= 1;
    }
    let mut p = 0.0;
    let a = 9.0 * k as f64 - 8.0;
    for s5 in 1..=9 {
        for t5 in 1..=9 {
            // 高橋君s5 青木君t5となるような確率
            if s5 == t5 {
                // 同じカードなら2枚残ってないとだめ
                if deck[&s5] < 2 {
                    continue;
                }
            } else {
                if deck[&s5] < 1 || deck[&t5] < 1 {
                    continue;
                }
            }
            let tkhs_point = point(&s, s5);
            let aoki_point = point(&t, t5);
            if tkhs_point > aoki_point {
                if s5 != t5 {
                    p += deck[&s5] as f64 * deck[&t5] as f64;
                } else {
                    p += (deck[&s5] as f64) * (deck[&s5] as f64 - 1.0);
                }
            }
        }
    }
    println!("{}", p / (a * (a - 1.0)));
}
