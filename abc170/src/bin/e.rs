#![allow(unused_imports, dead_code)]
use proconio::{input, fastout};
use std::cmp::*;
use std::collections::*;

const MAX: usize = 200_000;

struct SegTree<T>
where
    T: Copy,
{
    size: usize,
    seg: Vec<T>,
    f: fn(&T, &T) -> T,
    id: T,
}
impl<T> std::fmt::Debug for SegTree<T>
where
    T: Copy + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SegTree {{ ")?;
        write!(f, "size: {}, vec: {:?} }}", self.size, self.seg)?;
        Ok(())
    }
}
impl<T> SegTree<T>
where
    T: Copy,
{
    fn new(n: usize, f: fn(&T, &T) -> T, id: T) -> SegTree<T> {
        let mut size = 1;
        while n > size {
            size <<= 1;
        }
        let seg = vec![id; 2 * size];
        Self { size, seg, f, id }
    }
    fn set(&mut self, k: usize, v: T) {
        self.seg[k + self.size] = v;
    }
    fn get(&self, k: usize) -> T {
        self.seg[k + self.size]
    }
    fn build(&mut self) {
        for k in (1..self.size).rev() {
            self.seg[k] = (self.f)(&self.seg[2 * k + 0], &self.seg[2 * k + 1]);
        }
    }
    fn update(&mut self, k: usize, v: T) {
        let mut k = k + self.size;
        self.seg[k] = v;
        while k > 1 {
            self.seg[k >> 1] = (self.f)(&self.seg[k], &self.seg[k ^ 1]);
            k >>= 1;
        }
    }
    fn query(&self, i: usize, j: usize) -> T {
        let mut s = self.id;
        let mut l = i + self.size;
        let mut r = j + self.size;
        while l < r {
            if (l & 1) > 0 {
                s = (self.f)(&s, &self.seg[l]);
                l += 1;
            }
            if (r & 1) > 0 {
                s = (self.f)(&s, &self.seg[r - 1]);
            }
            l >>= 1;
            r >>= 1;
        }
        s
    }
}

#[fastout]
fn main() {
    input!(n: usize, q: usize);
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut gardens = vec![BTreeSet::<(u64, usize)>::new(); MAX];
    let mut highests = SegTree::<u64>::new(
        MAX, |&a, &b| min(a, b), std::u64::MAX
    );
    for i in 0..n {
        input!(ai: u64, bi: usize);
        a.push(ai);
        b.push(bi-1);
        gardens[bi-1].insert((ai, i));
        highests.update(bi-1, gardens[bi-1].iter().rev().next().unwrap().0);
    }
    for _ in 0..q {
        input!(ci: usize, di: usize);
        let ci = ci - 1;
        let to = di - 1;
        // 出る方
        let from = b[ci];
        gardens[from].remove(&(a[ci], ci));
        let v = {
            if gardens[from].is_empty() {
                std::u64::MAX
            } else {
                gardens[from].iter().rev().next().unwrap().0
            }
        };
        highests.update(from, v);
        // 入る方
        b[ci] = to;
        gardens[to].insert((a[ci], ci));
        highests.update(to, gardens[to].iter().rev().next().unwrap().0);
        println!("{}", highests.query(0, MAX+1));
    }
}
