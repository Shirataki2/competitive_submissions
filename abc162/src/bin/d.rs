#![allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, s: Chars);
    // 条件1をみたすもの
    let r = s.iter().filter(|&c| *c == 'R').count();
    let g = s.iter().filter(|&c| *c == 'G').count();
    let b = s.iter().filter(|&c| *c == 'B').count();
    let mut ans = r * g * b;
    // うち条件2を満たさないものを除外
    for i in 0..n-2 {
        for j in i+1..n-1 {
            let k = 2 * j - i;
            if k >= n { continue; }
            if s[i] != s[j] && s[j] != s[k] && s[k] != s[i] {
                ans -= 1;
            }
        }
    }
    println!("{}", ans);
}
