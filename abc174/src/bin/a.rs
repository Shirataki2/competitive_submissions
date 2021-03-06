use proconio::{input, fastout};
#[allow(unused_imports)]
use std::cmp::*;


#[fastout]
fn main() {
    input!(x: i32);
    println!("{}", if x >= 30 { "Yes" } else { "No" });
}
