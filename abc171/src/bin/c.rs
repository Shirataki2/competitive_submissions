use proconio::{input, fastout};
#[allow(unused_imports)]
use std::cmp::*;

#[fastout]
fn main() {
    input!(mut n: u64);
    let mut ans = String::from("");
    while n > 0 {
        n -= 1;
        let c = ('a' as u8 + (n % 26) as u8) as char;
        ans.push(c);
        n /= 26;
    }
    println!("{}", ans.chars().rev().collect::<String>());
}
