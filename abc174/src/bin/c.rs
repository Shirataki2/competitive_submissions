use proconio::{input, fastout};
#[allow(unused_imports)]
use std::cmp::*;

#[fastout]
fn main() {
    input!(k: u64);
    let mut a = 7;
    let mut ans = 1;
    for _ in 0..k {
        if a % k == 0 {
            println!("{}", ans);
            return
        }
        a = (10 * a + 7) % k;
        ans += 1;
    }
    println!("{}", -1)
}
