#![allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
use std::cmp::*;

#[fastout]
fn main() {
    input!(h: usize, w: usize, k: usize, s: [Chars; h]);
    let mut ans = 1usize<<60;
    // hが小さいので切り分け方でbit全探索
    for bit in 0..1<<(h-1) {
        // 縦i列目のグループ
        let mut group = vec![0; h];
        let mut ny = 0;
        for i in 0..h-1 {
            if bit >> i & 1 == 1 {group[i+1] = group[i] + 1; ny += 1; }
            else {group[i+1] = group[i]; }
        }
        // 縦グループごとの累計ホワイトチョコ
        // k を超えるまで足していきkを超えたらリセットする
        let mut whitex = vec![0usize; ny+1];
        let mut nx = 0;
        for x in 0..w {
            for y in 0..h {
                whitex[group[y]] += if s[y][x] == '1' { 1 } else { 0 };
            }
            // kより多いグループが存在
            if whitex.iter().any(|&x| x > k) {
                nx += 1;
                // rollback
                whitex = vec![0usize; ny+1];
                for y in 0..h {
                    whitex[group[y]] += if s[y][x] == '1' { 1 } else { 0 };
                }
            }
            // そもそも横1つでk個を超える組があったらその分割法は不適
            if whitex.iter().any(|&x| x > k) { nx = 1<<60; }
        }
        ans = min(ans, nx + ny);
    }
    println!("{}", ans);
}
