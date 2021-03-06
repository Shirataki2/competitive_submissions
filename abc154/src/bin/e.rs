#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;

fn main() {
    input!(n: Chars, k: usize);
    let ans = dfs(0, 0, false, &n, k);
    println!("{}", ans);
}

/// i桁目を走査
/// z回ゼロでない値が登場
/// small: 有意にnより小さいかどうか
/// n: 入力
fn dfs(i: usize, z: usize, small: bool, n: &[char], k: usize) -> i64 {
    if z == k {
        return 1;
    }
    if i == n.len() {
        return if z == k { 1 } else { 0 };
    }

    let mut r = 0;
    
    if small {
        // 有意にzより小さい
        // 1~9を選択
        r += 9 * dfs(i + 1, z + 1, true, n, k);
        // 0を選択
        r += dfs(i + 1, z, true, n, k);
    } else {
        // i桁目がdである
        let d = n[i].to_digit(10).unwrap() as i64;

        // 有意に小さいことが確定(d > 1)
        if d >= 1 {
            // ex) d = 5

            // d = 5
            r += dfs(i + 1, z + 1, false, n, k);
            // d = 4, 3, 2, 1
            r += (d - 1) * dfs(i + 1, z + 1, true, n, k);
            // d = 0
            r += dfs(i + 1, z, true, n, k);
        } else {
            // d = 0
            r += dfs(i + 1, z, false, n, k);
        }
    }
    r
}