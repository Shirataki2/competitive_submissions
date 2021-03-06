use proconio::marker::Chars;
use proconio::{input, fastout};
use std::cmp::min;

#[fastout]
fn main() {
    input!(s: Chars, t: Chars);
    let sl = s.len();
    let tl = t.len();
    let mut ans = tl;
    for i in 0..=sl-tl {
        let mut ctr = 0;
        for j in 0..tl {
            if s[i+j] != t[j] {
                ctr += 1;
            }
        }
        ans = min(ans, ctr);
    }
    println!("{}", ans);
}
