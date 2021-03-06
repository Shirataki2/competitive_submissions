use proconio::{input, fastout, marker::Chars};
use itertools::zip;
#[allow(unused_imports)]
use std::cmp::*;

#[fastout]
fn main() {
    input!(s: Chars, t: Chars);
    let mut ans = 0;
    for (c1, c2) in zip(&s, &t) {
        if c1 != c2 { ans += 1 }
    }
    println!("{}", ans);
}
