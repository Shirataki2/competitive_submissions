use proconio::{input, fastout};
#[allow(unused_imports)]
use std::cmp::*;

#[fastout]
fn main() {
    input!(a: u32);
    println!("{}", a+a*a+a*a*a);
}
