#![allow(unused_macros)]
macro_rules! input { ($io:expr => $($name:ident: $t:ty),+) => { $(let $name: $t = $io.read();)* }; }
macro_rules! outln { ($io: expr) => { $io.write("\n".to_string()); }; ($io: expr => $fmt: expr) => {$io.write(format!(concat!($fmt, "\n")))}; ($io: expr => $fmt: expr, $($arg: tt)*) => { $io.write(format!(concat!($fmt, "\n"), $($arg)*)); }; }
macro_rules! out { ($io: expr => $fmt: expr) => {$io.write(format!($fmt, "\n"))}; ($io: expr => $fmt: expr, $($arg: tt)*) => { $io.write(format!($fmt, $($arg)*)); }; }

use io::IO;
use sparse_table::*;

fn main() {
    let mut io = IO::new(std::io::stdin(), std::io::stdout());
    input!(io => n: usize, q: usize);
    let a: Vec<i64> = io.vec(n);
    let st = SparseTable::new(&a, OperationType::Min);
    for _ in 0..q {
        input!(io => l: usize, r: usize);
        outln!(io => "{}", a[st.query(l, r-1)]);
    }
}

pub mod sparse_table {
    #[derive(Debug)]
    pub struct SparseTable<T> {
        pub data: Vec<T>,
        table: Vec<Vec<usize>>,
        logs: Vec<usize>,
        op: OperationType,
    }

    #[derive(Debug)]
    pub enum OperationType {
        Min, Max
    }

    impl<T: Ord + Clone + Copy> SparseTable<T> {
        pub fn new(v: &[T], op: OperationType) -> Self {
            let n = v.len();
            let data = v.to_vec();
            let mut logs = vec![0; n+1];
            for i in 2..=n {
                logs[i] = logs[i >> 1] + 1;
            }
            let mut table = vec![vec![0; logs[n]+1]; n];
            for i in 0..n {
                table[i][0] = i;
            }
            for k in (1..n).take_while(|j| (1 << j) <= n) {
                for i in (0..n).take_while(|j| j + (1 << k) <= n) {
                    let v1 = table[i][k - 1];
                    let v2 = table[i + (1 << (k - 1))][k - 1];
                    let f = match op {
                        OperationType::Min => { data[v1] < data[v2] }
                        OperationType::Max => { data[v1] > data[v2] }
                    };
                    table[i][k] = if f { v1 } else { v2 };
                }
            }
            Self { data, table, logs, op }
        }

        /// Returns the index of the maximum/minimum value of
        /// the array in the **closed interval** [s..t].
        pub fn query(&self, s: usize, t: usize) -> usize {
            let d = t - s + 1;
            let k = self.logs[d];
            let (v1, v2) = (self.table[s][k], self.table[t + 1 - (1 << k)][k]);
            let f = match self.op {
                OperationType::Min => { self.data[v1] < self.data[v2] },
                OperationType::Max => { self.data[v1] > self.data[v2] },
            };
            if f { v1 } else { v2 }
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