use proconio::{input, fastout};

pub struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
    size: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).map(|n| n).collect(),
            sizes: vec![1; n],
            size: n
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
        let px = self.find(x);
        let py = self.find(y);
        if px == py { return false }
        let (large, small) = if self.sizes[px] > self.sizes[py] {
            (px, py)
        } else {
            (py, px)
        };
        self.parent[small] = large;
        self.sizes[large] += self.sizes[small];
        self.sizes[small] = 0;
        self.size -= 1;
        true
    }
}

#[fastout]
fn main() {
    input!(n: usize, m: usize);
    input!(edges: [(usize, usize); m]);
    let mut uf = UnionFind::new(n);
    for (a, b) in edges {
        uf.unite(a-1, b-1);
    }
    if let Some(ans) = uf.sizes.iter().max() {
        println!("{}", ans);
    } else {
        println!("{}", 0);
    }
}
