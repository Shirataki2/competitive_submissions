#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;

use procon_lazy_segtree::{LazySegTree, MaxAdd};

const CANVAS_SIZE: usize = 10000;

fn main() {
    //! 方針
    //! できるだけ左上に詰めていく
    //! 長方形のサイズはなるべく縦長，ただしほかの広告のy座標にぶつからない程度で
    //! 決める必要がある．
    input!(n: usize);
    let mut ads = Vec::with_capacity(n);
    for i in 0..n {
        input!(x: i64, y: i64, r: i64);
        ads.push(Ad { i, x, y, r, coord: (0, 0, 0, 0) });
    }
    ads.sort();
    println!("{:?}", ads);
    let mut solver = Solver::new(ads);
    let ans = solver.put_all();
    for a in ans.iter() {
        println!("{} {} {} {}", a.0, a.1, a.2, a.3);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Ad {
    i: usize,
    x: i64,
    y: i64,
    r: i64,
    coord: (i64, i64, i64, i64)
}

impl PartialOrd for Ad {
    fn partial_cmp(&self, other: &Ad) -> Option<Ordering> {
        (self.x + self.y).partial_cmp(&(other.x + other.y))
    }
}

impl Ord for Ad {
    fn cmp(&self, other: &Ad) -> Ordering {
        (self.x + self.y).cmp(&(other.x + other.y))
    }
}

struct Solver {
    ads: Vec<Ad>,
    left_margin: LazySegTree<MaxAdd<i64>>,
    up_margin: LazySegTree<MaxAdd<i64>>,
}

impl Solver {
    fn new(ads: Vec<Ad>) -> Solver {
        let left_margin = vec![0; CANVAS_SIZE].into();
        let up_margin = vec![0; CANVAS_SIZE].into();
        Solver {
            ads,
            left_margin,
            up_margin,
        }
    }

    fn put_all(&mut self) -> Vec<(i64, i64, i64, i64)> {
        for i in 0..self.ads.len() {
            self.put(i);
        }
        let mut ret = vec![(0, 0, 0, 0); self.ads.len()];
        for ad in self.ads.iter() {
            ret[ad.i] = ad.coord;
        }
        ret
    }

    fn put(&mut self, i: usize) {
        let ad = self.ads[i];
        let (_xj, yj) = self.search(i);
        // 左上
        let xlu = self.left_margin.query((yj - ad.y) as usize..min(yj as usize, CANVAS_SIZE));
        let h = yj - ad.y;
        let mut f = 1.0;
        let mut ylu;
        let mut w ;
        loop {
            w = min(CANVAS_SIZE as i64, (ad.r as f64 * f / h as f64) as i64);
            println!("{} {} {} {:?} {}", i, yj, xlu, ad, f);
            ylu = self.up_margin.query(xlu as usize..(xlu + w) as usize) as usize;
            if self.check_w(i, xlu as i64, ylu as i64, h, w) {
                break
            }
            f *= 0.9;
        }
        self.left_margin.apply_range(ylu..ylu+h as usize, w);
        self.up_margin.apply_range(xlu as usize..(xlu+w) as usize, h);
        self.ads[i].coord = (xlu, ylu as i64, w, h);
    }

    fn check_w(&self, i: usize, xlu: i64, ylu: i64, h: i64, w: i64) -> bool {
        for j in 0..self.ads.len() {
            if i == j {
                continue;
            }
            if self.ads[j].x < xlu + w && self.ads[j].x > xlu && self.ads[j].y < ylu + h && self.ads[j].y > ylu {
                return false;
            }
        }
        true
    }

    fn search(&self, i: usize) -> (i64, i64) {
        let mut d = 1_000_000_000_000_000i64;
        let mut a = self.ads.len() - 1;
        for j in 0..self.ads.len() {
            if self.ads[j].x < self.ads[i].x || self.ads[j].y < self.ads[i].y {
                continue;
            }
            if i == j {
                continue;
            }
            let dd = (self.ads[i].x - self.ads[j].x).pow(2) + (self.ads[i].y - self.ads[j].y).pow(2);
            if dd < d {
                a = j;
                d = dd;
            }
        }
        (self.ads[a].x, self.ads[a].y)
    }
}