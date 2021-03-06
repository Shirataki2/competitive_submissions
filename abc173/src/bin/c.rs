use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::cmp::*;

#[fastout]
fn main() {
    input!(h: usize, w: usize, k: usize, c: [Chars; h]);
    let mut ans = 0;
    for by in 0..1<<h {
        for bx in 0..1<<w {
            let mut b = 0;
            for y in 0..h {
                for x in 0..w {
                    let fy = ((by >> y) & 1) == 0;
                    let fx = ((bx >> x) & 1) == 0;
                    if fy && fx && c[y][x] == '#' { b += 1;}
                }
            }
            if b == k { ans += 1; }
        }
    }
    println!("{}", ans);
}
