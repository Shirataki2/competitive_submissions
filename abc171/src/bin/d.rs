use proconio::{input, fastout};
#[allow(unused_imports)]
use std::cmp::*;
use std::collections::HashMap;

#[fastout]
fn main() {
    input!(n: usize, a: [i64; n]);
    let mut map = HashMap::<i64, i64>::new();
    let mut ans = 0;
    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
        ans += a[i];
    }
    input!(q: usize);
    for _ in 0..q {
        input!(b: i64, c: i64);
        if map.contains_key(&b) {
            ans += (c - b) * map[&b];
            *map.entry(c).or_insert(0) += map[&b];
            *map.entry(b).or_default() = 0;
        }
        println!("{}", ans);
    }
}
