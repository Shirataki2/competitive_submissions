#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

enum Query {
    And,
    Or
}

#[fastout]
fn main() {
    input!(n: usize);
    let mut s = Vec::with_capacity(n);
    for _ in 0..n {
        input!(v: String);
        s.push(if &v == "AND" { Query::And } else { Query::Or });
    }
    let mut dp = vec![0u64; n + 1];
    dp[0] = 1;
    // ORの場合は 2 ^ N (True) + dp[N - 1] (False)
    for i in 0..n {
        match s[i] {
            Query::And => {
                dp[i+1] = dp[i];
            }
            Query::Or => {
                dp[i+1] = (1 << i + 1) + dp[i];
            }
        }
    }
    println!("{}", dp[n]);
}
