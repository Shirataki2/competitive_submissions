#![allow(unused_macros)]
use io::IO;
use unionfind::UnionFind;
macro_rules! input { ($io:expr => $($name:ident: $t:ty),+) => { $(let $name: $t = $io.read();)* }; }
macro_rules! outln { ($io: expr) => { $io.write("\n".to_string()); }; ($io: expr => $fmt: expr) => {$io.write(format!(concat!($fmt, "\n")))}; ($io: expr => $fmt: expr, $($arg: tt)*) => { $io.write(format!(concat!($fmt, "\n"), $($arg)*)); }; }
macro_rules! out { ($io: expr => $fmt: expr) => {$io.write(format!($fmt, "\n"))}; ($io: expr => $fmt: expr, $($arg: tt)*) => { $io.write(format!($fmt, $($arg)*)); }; }

fn main() {
    let mut io = IO::new(std::io::stdin(), std::io::stdout());
    input!(io => n: usize, q: usize);
    let mut uf = UnionFind::new(n);
    for _ in 0..q {
        input!(io => t: usize, u: usize, v: usize);
        match t {
            0 => {
                uf.unite(u, v);
            },
            _ => {
                outln!(io => "{}", if uf.is_same(u, v) { 1 } else { 0 });
            }
        }
    }
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

pub mod io {
    use std::io::{Read, Write};
    pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

    impl<R, W> IO<R, W>
    where
        R: Read,
        W: Write,
    {
        /// To use standard I/O
        /// 
        /// `IO::new(std::io::stdin(), std::io::stdout())`
        pub fn new(r: R, w: W) -> Self {
            Self(r, std::io::BufWriter::new(w))
        }

        pub fn write<S: ToString>(&mut self, s: S) {
            self.1.write_all(s.to_string().as_bytes()).unwrap();
        }

        pub fn read<T: std::str::FromStr>(&mut self) -> T {
            let buf = 
                self
                .0.by_ref()
                .bytes()
                .map(|b| b.unwrap())
                .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\t' || b == b'\r')
                .take_while(|&b| b != b' ' && b != b'\n' && b != b'\t' && b != b'\r')
                .collect::<Vec<_>>();
            unsafe { std::str::from_utf8_unchecked(&buf) }
                .parse()
                .ok()
                .expect("Parse Error")
        }

        pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
            (0..n).map(|_| self.read()).collect()
        }

        pub fn chars(&mut self) -> Vec<char> {
            self.read::<String>().chars().collect()
        }
    }
}