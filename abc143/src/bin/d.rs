#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, mut l: [usize; n]);
    l.sort();
    l.push(1<<62);
    let mut ans = 0;
    for a in 0..n-2 {
        for b in a+1..n-1 {
            let x = l[a] + l[b];
            let c = l.lower_bound(&x);
            ans += c - b - 1;
        }
    }
    println!("{}", ans);
}

pub trait BinarySearch<T: Ord> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut left = 0;
        let mut right = self.len();
        while left != right {
            let mid = (left + right) / 2;
            match self[mid].cmp(&x) {
                std::cmp::Ordering::Less => {
                    left = mid + 1;
                },
                _ => {
                    right = mid;
                }
            }
        }
        left
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut left = 0;
        let mut right = self.len();
        while left != right {
            let mid = (left + right) / 2;
            match self[mid].cmp(&x) {
                std::cmp::Ordering::Greater => {
                    right = mid;
                },
                _ => {
                    left = mid + 1;
                }
            }
        }
        left
    }
}