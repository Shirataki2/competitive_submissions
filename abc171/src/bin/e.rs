use proconio::{input, fastout};
#[allow(unused_imports)]
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, a: [u64; n]);
    let sa = a.iter().fold(0, |acc, &x| acc ^ x);
    let x = (0..n).map(|i| sa ^ a[i]).collect::<Vec<u64>>();
    for i in 0..n-1 {
        print!("{} ", x[i]);
    }
    println!("{}", x[n-1]);
}
