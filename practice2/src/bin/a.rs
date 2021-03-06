#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;

use procon_unionfind::UnionFind;

fn main() {
    input!(n: usize);
    input!(q: usize);
    let mut uf = UnionFind::new(n);

    for _ in 0..q {
        input!(t: usize, u: usize, v: usize);
        match t {
            0 => {
                uf.unite(u, v);
            }
            1 => {
                println!("{}", if uf.is_same(u, v) { 1 } else { 0 });
            }
            _ => unreachable!(""),
        }
    }
}
