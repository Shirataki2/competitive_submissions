use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::cmp::*;

#[fastout]
fn main() {
    // 一番右の赤と一番左の白を入れ替える動作を続ければ良い
    input!(n: i32, s: Chars);
    let mut l: i32 = 0;
    let mut r: i32 = (n - 1) as i32;
    // R: 1, W: 0
    let mut a = s.iter().map(|&c| if c == 'R' { 1 } else { 0 }).collect::<Vec<i32>>();
    let mut ans = 0;
    loop {
        while l < n && a[l as usize] != 0 {
            l += 1;
        }
        while r >= 0 && a[r as usize] != 1 {
            r -= 1;
        }
        if l == n || r == -1 || l >= r {
            break;
        }
        a[l as usize] = 1;
        a[r as usize] = 0;
        ans += 1;
    }
    println!("{}", ans);
}
