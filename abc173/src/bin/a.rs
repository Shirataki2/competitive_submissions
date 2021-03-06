use proconio::{input, fastout};
#[allow(unused_imports)]
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: i32);
    for i in 1..=10 {
        if i * 1000 - n >= 0 {
            println!("{}", i * 1000 - n);
            break
        }
    }
}
