#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;
use lazy_segtree::*;

#[fastout]
fn main() {
    input!(n: usize, d: usize, a: i64);
    input!(mut enemies: [(usize, i64); n]);
    enemies.sort_by_key(|e| e.0);
    let x = enemies.iter().map(|e| e.0).collect::<Vec<_>>();
    let mut h = enemies.iter().map(|e| e.1).collect::<Vec<_>>();

    // 左端から爆破させていく
    // 昇順に敵を並べ届く敵の右端のindexを尺取り法で取得
    // HPを減らすには遅延セグメント木を使う

    for _ in 0..10 { h.push(0); }

    let mut seg = LazySegTree::new(
        h.clone(),
        |&a, &b| a + b,
        |&a, &b| a + b,
        |&a, &b| a + b,
        0, 0
    );
    let mut ans = 0;
    let mut right = 0;
    for left in 0..n {
        let cur = seg.query(left, left+1);
        if cur <= 0 { continue; }
        // 最も右の攻撃対象を求める
        while right + 1 < n && x[right + 1] - x[left] <= 2 * d { right += 1; }

        // 一番左の敵を倒すのに必要な攻撃回数
        let need = (cur + a - 1) / a;

        seg.update(left, right + 1, -need*a);

        ans += need;
    }
    println!("{}", ans);
}

pub mod lazy_segtree {
    /// ## Cheetsheet
    /// 
    /// ### Range Minimum Query + Range Add Query
    /// ```no_test
    /// v: [0; n]
    /// f(a, b) = min(a, b)
    /// g(a, b) = a + b
    /// h(a, b) = a + b
    /// t0 = inf, u0 = 0
    /// ```
    /// 
    /// ### Range Sum Query + Range Add Query
    /// ```no_test
    /// v: [(0, 1); n]
    /// f(a, b) = (a.0 + b.0, a.1 + b.1)
    /// g(a, b) = (a.0 + b * a.1, a.1)
    /// h(a, b) = a + b
    /// t0 = inf, u0 = 0
    /// ```
    pub struct LazySegTree<T, U>
    {
        size: usize,
        height: usize,
        pub data: Vec<T>,
        pub lazy: Vec<U>,
        f: fn(&T, &T) -> T,
        g: fn(&T, &U) -> T,
        h: fn(&U, &U) -> U,
        t0: T,
        u0: U,
    }

    impl<T, U> LazySegTree<T, U>
    where
        T: Clone + Copy,
        U: Clone + Copy + Eq,
    {
        pub fn new(v: Vec<T>, f: fn(&T, &T) -> T, g: fn(&T, &U) -> T, h: fn(&U, &U) -> U, t0: T, u0: U) -> Self {
            let size = v.len();
            let size = size.next_power_of_two();
            // height = log_2 size
            let height = {
                let mut v = 0;
                let mut s = size;
                while s > 0 { s >>= 1; v += 1; }
                v
            };
            let mut data = vec![t0; 2 * size];
            data[size..(v.len() + size)].clone_from_slice(&v[..]);
            let lazy = vec![u0; 2 * size];
            for k in (1..size).rev() {
                data[k] = (f)(&data[2 * k], &data[2 * k + 1]);
            }
            Self { size, height, data, lazy, f, g, h, t0, u0 }
        }

        pub fn set(&mut self, k: usize, v: T) {
            self.data[k + self.size] = v;
        }

        pub fn build(&mut self) {
            for k in (1..self.size).rev() {
                self.data[k] = (self.f)(&self.data[2 * k], &self.data[2 * k + 1]);
            }
        }

        #[inline]
        fn propagate(&mut self, k: usize) {
            if self.lazy[k] != self.u0 {
                self.lazy[2 * k] = (self.h)(&self.lazy[2 * k], &self.lazy[k]);
                self.lazy[2 * k + 1] = (self.h)(&self.lazy[2 * k + 1], &self.lazy[k]);
                self.data[k] = self.reflect(k);
                self.lazy[k] = self.u0;
            }
        }

        #[inline]
        fn reflect(&mut self, k: usize) -> T {
            if self.lazy[k] == self.u0 {
                self.data[k]
            } else {
                (self.g)(&self.data[k], &self.lazy[k])
            }
        }

        #[inline]
        fn recalc(&mut self, mut k: usize) {
            k >>= 1;
            while k > 0 {
                self.data[k] = (self.f)(&self.reflect(2 * k), &self.reflect(2 * k + 1));
                k >>= 1;
            }
        }

        #[inline]
        fn thrust(&mut self, k: usize) {
            for i in (1..=self.height).rev() {
                self.propagate(k >> i);
            }
        }

        pub fn update(&mut self, mut left: usize, mut right: usize, value: U) {
            left += self.size;
            right += self.size - 1;
            self.thrust(left);
            self.thrust(right);
            let mut l = left; let mut r = right + 1;
            while l < r {
                if l & 1 > 0 {
                    self.lazy[l] = (self.h)(&self.lazy[l], &value);
                    l += 1;
                }
                if r & 1 > 0 {
                    r -= 1;
                    self.lazy[r] = (self.h)(&self.lazy[r], &value);
                }
                l >>= 1; r >>= 1;
            }
            self.recalc(left);
            self.recalc(right);
        }

        pub fn query(&mut self, mut a: usize, mut b: usize) -> T {
            a += self.size;
            b += self.size - 1;
            self.thrust(a);
            self.thrust(b);
            let mut l = a; let mut r = b + 1;
            let mut lv = self.t0; let mut rv = self.t0;
            while l < r {
                if l & 1 > 0 {
                    lv = (self.f)(&lv, &self.reflect(l));
                    l += 1;
                }
                if r & 1 > 0 {
                    r -= 1;
                    rv = (self.f)(&self.reflect(l), &rv);
                }
                l >>= 1; r >>= 1;
            }
            (self.f)(&lv, &rv)
        }
    }
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