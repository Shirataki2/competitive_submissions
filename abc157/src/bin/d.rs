#![allow(unused_imports)]
use proconio::{input, fastout, marker::Usize1};
use std::cmp::*;
use unionfind::*;

//#[fastout]
fn main() {
    //! 友達関係でUnionFind木を作る
    //! 1 - 2 = 3
    //!     |-4-|
    //! (=はブロック関係)を考える．
    //! 連結成分は4であり，2と友人候補になりうるのは3であるが
    //! 2と3はブロック関係にある．
    //! したがって連結成分内で自身とブロック関係にある頂点を高速に求めればよい
    input!(n: usize, m: usize, k: usize);
    let mut uf = UnionFind::new(n);
    let mut friend = vec![vec![]; n];
    let mut blocked = vec![vec![]; n];
    for _ in 0..m {
        input!(a: Usize1, b: Usize1);
        uf.unite(a, b);
        friend[a].push(b);
        friend[b].push(a);
    }
    for _ in 0..k {
        input!(c: Usize1, d: Usize1);
        if uf.is_same(c, d) {
            blocked[c].push(d);
            blocked[d].push(c);
        }
    }
    let mut ans = vec![];
    for i in 0..n {
        let p = uf.find(i);
        let f = uf.sizes[p] - 1 - friend[i].len() - blocked[i].len();
        ans.push(f);
    }
    println!("{}", ans.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}

pub mod unionfind {
    pub struct UnionFind {
        pub parent: Vec<usize>,
        pub sizes: Vec<usize>,
        pub size: usize,
    }

    impl UnionFind {
        pub fn new(n: usize) -> Self {
            Self {
                parent: (0..n).collect(),
                sizes: vec![1; n],
                size: n,
            }
        }

        pub fn find(&mut self, x: usize) -> usize {
            if x == self.parent[x] {
                x
            } else {
                let p = self.parent[x];
                self.parent[x] = self.find(p);
                self.parent[x]
            }
        }

        pub fn unite(&mut self, x: usize, y: usize) -> bool {
            let (px, py) = (self.find(x), self.find(y));
            if px == py { return false }
            let (l, r) = if self.sizes[px] > self.sizes[py] {
                (px, py)
            } else {
                (py, px)
            };
            self.parent[l] = r;
            self.sizes[r] += self.sizes[l];
            self.sizes[l] = 0;
            self.size -= 1;
            true
        }

        pub fn is_same(&mut self, x: usize, y: usize) -> bool {
            let (px, py) = (self.find(x), self.find(y));
            px == py
        }
    }
}
