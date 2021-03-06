#![allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
use std::cmp::*;

#[fastout]
fn main() {
    input!(mut s: Chars);
    s.reverse();
    let mut v = 0;
    let mut b = 1;
    let mut ctr = vec![0; 2019];
    ctr[0] = 1;
    let mut ans = 0;
    for c in s.iter() {
        v += c.to_digit(10).unwrap() as i64 * b;
        v %= 2019;
        ans += ctr[v as usize];
        ctr[v as usize] += 1;
        b *= 10;
        b %= 2019;
    }
    println!("{}", ans);
}

pub trait Accumulate: Iterator {
    fn accumulate<T>(self, v0: T, f: fn(&T, &Self::Item) -> T) -> AccumulateItertor<Self, T>
    where Self: Sized
    {
        AccumulateItertor { sum: v0, func: f, iter: self }
    }
}
impl<I: ?Sized> Accumulate for I where I: Iterator {}
pub struct AccumulateItertor<I: Iterator, T> {
    sum: T,
    func: fn(&T, &I::Item) -> T,
    iter: I,
}
impl<I, T> Iterator for AccumulateItertor<I, T>
where
    I: Iterator,
    T: Clone
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|v| {
            let v = (self.func)(&self.sum, &v);
            self.sum = v.clone();
            v
        })
    }
}

pub trait BitBruteForce: Iterator {
    fn bit_brute(self) -> BitBruteForceIterator<Self>
    where Self: Sized
    {
        BitBruteForceIterator { vec: self.collect(), mask: 0 }
    }
}
impl<I: ?Sized> BitBruteForce for I where I: Iterator {}
pub struct BitBruteForceIterator<I: Iterator> {
    vec: Vec<I::Item>,
    mask: usize,
}
impl<I> Iterator for BitBruteForceIterator<I>
where
    I: Iterator,
    I::Item: Clone + Copy + Sized
{
    type Item = Vec::<I::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.vec.len();
        if self.mask < (1 << n) {
            let bit_n = self.mask.count_ones() as usize;
            let mut v = Vec::with_capacity(bit_n);
            for i in 0..n {
                if self.mask >> i & 1 == 1 {
                    v.push(self.vec[i])
                }
            }
            self.mask += 1;
            Some(v)
        } else {
            None
        }
    }
}

pub trait Combinations: Iterator {
    fn combinations(self, r: usize) -> CombinationsIterator<Self> where Self: Sized {
        let indices = (0..r).collect();
        CombinationsIterator { vec: self.collect(), indices, r, first: true }
    }
    fn combinations_with_replacement(self, r: usize) -> CombinationsWithReplacementIterator<Self> where Self: Sized {
        let indices = vec![0; r];
        CombinationsWithReplacementIterator { vec: self.collect(), indices, r, first: true }
    }
}
impl<I: ?Sized> Combinations for I where I: Iterator {}
pub struct CombinationsIterator<I: Iterator> {
    vec: Vec<I::Item>,
    indices: Vec<usize>,
    r: usize,
    first: bool
}
pub struct CombinationsWithReplacementIterator<I: Iterator> {
    vec: Vec<I::Item>,
    indices: Vec<usize>,
    r: usize,
    first: bool
}
impl<I> Iterator for CombinationsIterator<I>
where
    I: Iterator,
    I::Item: Sized + Copy
{
    type Item = Vec<I::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.vec.len();
        let r = self.r;
        if n < r { return None }
        if self.first {
            self.first = false;
        } else {
            let mut i = r - 1;
            while self.indices[i] == i + n - r {
                if i > 0 { i -= 1; } else { return None }
            }
            self.indices[i] += 1;
            for j in i+1..r {
                self.indices[j] = self.indices[j - 1] + 1;
            }
        }
        Some(self.indices.iter().map(|&i| self.vec[i]).collect())
    }
}
impl<I> Iterator for CombinationsWithReplacementIterator<I>
where
    I: Iterator,
    I::Item: Sized + Copy
{
    type Item = Vec<I::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.vec.len();
        let r = self.r;
        if self.first {
            self.first = false;
        } else {
            let mut i = r - 1;
            while self.indices[i] == n - 1 {
                if i > 0 { i -= 1; } else { return None }
            }
            let v = self.indices[i];
            for j in i..r {
                self.indices[j] = v + 1;
            }
        }
        Some(self.indices.iter().map(|&i| self.vec[i]).collect())
    }
}