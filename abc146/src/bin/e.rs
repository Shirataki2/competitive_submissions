#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;
use std::collections::*;

#[fastout]
fn main() {
    //! S_i = cumsum(A_i)
    //! (S_j - S_i) % K = j - i
    //! S_j - j \equiv S_i - i (i < j)
    //! T_i := S_i - i
    //! 
    //! A_i:   1 4 2  3  5
    //! S_i:   1 5 7 10 15
    //! T_i: 0 0 3 4  6 10
    //!    : 0 0 3 0  2  2
    //! 
    //! i を固定して考えると，[i, i+k-1]の範囲でS_iと同じ値の個数を求める
    input!(n: usize, k: usize, mut a: [i64; n]);
    let mut s = vec![0];
    for ai in a.iter() { s.push(s.last().unwrap() + ai); }
    for i in 0..s.len() { s[i] -= i as i64; s[i] %= k as i64; }
    let mut ans = 0;
    let mut map: HashMap<i64, i64> = HashMap::new();
    for j in 0..n+1 {
        if j >= k {
            *map.get_mut(&s[j-k]).unwrap() -= 1;
        }
        ans += map.get(&s[j]).unwrap_or(&0);
        *map.entry(s[j]).or_insert(0) += 1;
    }
    println!("{}", ans);
}
