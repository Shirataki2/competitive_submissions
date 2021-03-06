#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;

fn main() {
    input!(h: usize, w: usize, s: [Chars; h]);
    let mut dp = vec![vec![100000; w * h]; w * h];
    for i in 0..w*h {
        dp[i][i] = 0;
    }
    let h = h as i32;
    let w = w as i32;
    for y in 0..h {
        for x in 0..w {
            for &(dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if y + dy >= h || x + dx >= w || x + dx < 0 || y + dy < 0 {
                    continue;
                }
                if s[y as usize][x as usize] == '#' {
                    continue;
                }
                if s[(y + dy) as usize][(x + dx) as usize] == '.' {
                    dp[(y * w + x) as usize][((y + dy) * w + x + dx) as usize] = 1;
                    dp[((y + dy) * w + x + dx) as usize][(y * w + x) as usize] = 1;
                }
            }
        }
    }
    let h = h as usize;
    let w = w as usize;
    for k in 0..w*h {
        for i in 0..w*h {
            for j in 0..w*h {
                dp[i][j] = min(dp[i][j], dp[i][k] + dp[k][j]);
            }
        }
    }
    let mut ans = 0;
    for y in 0..w*h {
        for x in 0..w*h {
            if dp[y][x] < 100000 {
                ans = max(ans, dp[y][x]);
            }
        }
    }
    println!("{}", ans);
}
