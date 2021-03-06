use proconio::{input, fastout};
#[allow(unused_imports)]
use std::cmp::*;

#[fastout]
fn main() {
    input!(a: char);
    println!("{}", if a.is_uppercase() { "A" } else { "a" });
}
