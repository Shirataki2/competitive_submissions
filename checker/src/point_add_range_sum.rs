#![allow(unused_macros)]
macro_rules! input { ($io:expr => $($name:ident: $t:ty),+) => { $(let $name: $t = $io.read();)* }; }
macro_rules! outln { ($io: expr) => { $io.write("\n".to_string()); }; ($io: expr => $fmt: expr) => {$io.write(format!(concat!($fmt, "\n")))}; ($io: expr => $fmt: expr, $($arg: tt)*) => { $io.write(format!(concat!($fmt, "\n"), $($arg)*)); }; }
macro_rules! out { ($io: expr => $fmt: expr) => {$io.write(format!($fmt, "\n"))}; ($io: expr => $fmt: expr, $($arg: tt)*) => { $io.write(format!($fmt, $($arg)*)); }; }

use io::IO;
use segtree::SegTree;

fn main() {
    let mut io = IO::new(std::io::stdin(), std::io::stdout());
    input!(io => n: usize, q: usize);
    let mut st = SegTree::new(n, |&a, &b| a + b, 0);
    for i in 0..n {
        input!(io => a: usize);
        st.update(i, a);
    }
    for _ in 0..q {
        input!(io => t: usize, a: usize, b: usize);
        match t {
            0 => {
                st.update(a, st.get(a) + b);
            },
            _ => {
                let v = st.query(a, b);
                outln!(io => "{}", v);
            }
        }
    }
}

pub mod segtree {
    pub struct SegTree<T>
    where
        T: Copy,
    {
        size: usize,
        data: Vec<T>,
        f: fn(&T, &T) -> T,
        id: T,
    }

    impl<T> SegTree<T>
    where
        T: Copy,
    {
        pub fn new(n: usize, f: fn(&T, &T) -> T, id: T) -> SegTree<T> {
            let mut size = 1;
            while n > size { size <<= 1; }
            let data = vec![id; 2*size];
            Self { size, data, f, id }
        }

        pub fn set(&mut self, k: usize, v: T) {
            self.data[k + self.size] = v;
        }

        pub fn get(&self, k: usize) -> T {
            self.data[k + self.size]
        }

        pub fn build(&mut self) {
            for k in (1..self.size).rev() {
                self.data[k] = (self.f)(&self.data[2 * k + 0], &self.data[2 * k + 1]);
            }
        }

        pub fn update(&mut self, k: usize, v: T) {
            let mut k = k + self.size;
            self.data[k] = v;
            while k > 1 {
                self.data[k >> 1] = (self.f)(&self.data[k], &self.data[k^1]);
                k >>= 1;
            }
        }

        pub fn query(&self, i: usize, j: usize) -> T {
            let mut s = self.id;
            let mut l = i + self.size;
            let mut r = j + self.size;
            while l < r {
                if (l & 1) > 0 {
                    s = (self.f)(&s, &self.data[l]);
                    l += 1;
                }
                if (r & 1) > 0{
                    s = (self.f)(&s, &self.data[r - 1]);
                }
                l >>= 1;
                r >>= 1;
            }
            s
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
