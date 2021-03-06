#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
/// 隣り合うのが全くない(k=0)
/// M * (M - 1) ^ (N - 1)
/// 
/// 隣り合うのがi個
/// 1. 隣り合ったブロックをひと固まりと見ればN-i個のブロックを置く問題となる
/// 2. 色の選び方: 左端は好きな色置いて他はM-1色から選ぶ
/// 3. 置き方の選び方: comb(N-1, i)通り
/// 
/// \sum_{i}^{K} M * comb(N - 1, i) * (M - 1) ^ (N - (i - 1))
fn main() {
    input!(s: String);
    println!("{}", s);
}
