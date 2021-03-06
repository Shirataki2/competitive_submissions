use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(r: usize, c: usize, k: usize);
    // 1-origin
    let mut map = vec![vec![0u64; c+1]; r+1];
    for _ in 0..k {
        input!(y: usize, x: usize, v: u64);
        map[y][x] = v;
    }
    let mut dp = vec![vec![vec![0u64; 4]; c+1]; r+1];
    for y in 0..=r {
        for x in 0..=c {
            for k in 0..=3 {
                // ↓へ行くパターン
                if y >= 1 {
                    dp[y][x][0] = max(dp[y][x][0], dp[y-1][x][k]);
                    dp[y][x][1] = max(dp[y][x][1], dp[y-1][x][k] + map[y][x]);
                }
                // →へ行くパターン
                if x >= 1 {
                    dp[y][x][k] = max(dp[y][x][k], dp[y][x-1][k]);
                    if k >= 1 {
                        dp[y][x][k] = max(dp[y][x][k], dp[y][x-1][k-1] + map[y][x]);
                    }
                }
            }
        }
    }
    println!("{}", dp[r][c].iter().max().unwrap());
}
