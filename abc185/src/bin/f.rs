#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;

use procon::{segtree::SegTree, math_traits::*};

fn main() {
    input!(n: usize, q: usize, a: [u64; n]);
    let mut seg: SegTree<Xor<_>> = a.into();
    for _ in 0..q {
        input!(t: usize, x: Usize1, y: Usize1);
        match t {
            1 => {
                let v = seg.get(x) ^ (y + 1) as u64;
                seg.set(x, v);
            },
            _ => {
                let v = seg.query(x..=y);
                println!("{}", v);
            }
        }
    }
}
