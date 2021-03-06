use proconio::{input, fastout};
#[allow(unused_imports)]
use std::cmp::*;

#[fastout]
fn main() {
    const MAX: usize = 10_000_000;
    input!(n: usize);
    let mut arr = vec![1; MAX+ 1];
    for i in 2..=MAX {
        for j in 1.. {
            if i * j > MAX { break; }
            arr[i * j] += 1;
        }
    }
    let ans = (1..=n).fold(0, |acc, i| acc + i * arr[i]);
    println!("{}", ans);
}
