use proconio::input;
use std::cmp::*;

fn main() {
    input!(x: u64, y: u64, a: u64, b: u64);
    let mut ans = 0;
    let mut ctr = 0;
    let mut x = x;
    while x < y {
        let n = ctr + (y - 1 - x) / b;
        ans = max(ans, n);
        if x >= b || x >= y { break; }
        x *= a;
        ctr += 1;
    }
    println!("{}", ans);
}
