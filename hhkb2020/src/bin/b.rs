#![allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
use std::cmp::*;

#[fastout]
fn main() {
    input!(h: usize, w: usize, s: [Chars; h]);
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            // 横に敷く
            if j < w - 1 && s[i][j] == '.' && s[i][j+1] == '.' {
                ans += 1;
            }
            // 縦に敷く
            if i < h - 1 && s[i][j] == '.' && s[i+1][j] == '.' {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
