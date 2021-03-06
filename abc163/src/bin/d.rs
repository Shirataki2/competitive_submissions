#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    //! 各k \in [K, N+1]に対しての部分列の和は重複しない
    //! (k*10^100 + 端数 という形式になる)
    //! 
    //! k個の数を選ぶ通りの数は最大-最小+1で求まる
    //! 最小: 0+1+...+k-1 = k(k-1)/2
    //! 最大: N+N-1+...+N-k+1 = k(2N-k+1)/2
    const MOD: i64 = 1_000_000_007;
    input!(n: i64, k: i64);
    let mut ans: i64 = 0;
    for ki in k..=n+1 {
        let first = ki * (ki - 1) / 2;
        let last = ki * (2 * n - ki + 1) / 2;
        ans += (last - first + 1) % MOD;
    }
    println!("{}", ans % MOD);
}
