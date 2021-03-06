use io::IO;
use topological_sort::TopologicalSortGraph;

fn main() {
    let mut io = IO::new(std::io::stdin(), std::io::stdout());
    let (v, e): (usize, usize) = (io.read(), io.read());
    let mut g = TopologicalSortGraph::new(v);
    for _ in 0..e {
        let (s, t): (usize, usize) = (io.read(), io.read());
        g.add_edge(s, t);
    }
    io.write(format!("{}\n", match g.sort() {
        Some(_) => 0,
        None    => 1,
    }));
}

pub mod topological_sort {
    use std::collections::VecDeque;

    #[derive(Debug)]
    pub struct TopologicalSortGraph {
        graph: Vec<Vec<usize>>,
        degree: Vec<isize>,
    }

    impl TopologicalSortGraph {
        pub fn new(size: usize) -> Self {
            let graph = vec![vec![]; size];
            let degree = vec![0; size];
            Self { graph, degree }
        }

        pub fn add_edge(&mut self, from: usize, to: usize) {
            self.graph[from].push(to);
            self.degree[to] += 1;
        }

        pub fn sort(&mut self) -> Option<Vec<usize>> {
            let mut q = VecDeque::new();
            let mut ret = vec![];
            for i in 0..self.graph.len() {
                if self.degree[i] == 0 {
                    q.push_back(i);
                }
            }
            while !q.is_empty() {
                let v = q.pop_front().unwrap();
                ret.push(v);
                for to in self.graph[v].iter() {
                    self.degree[*to] -= 1;
                    if self.degree[*to] == 0 {
                        q.push_back(*to);
                    }
                }
            }
            match self.degree.iter().max().unwrap() {
                &0 => Some(ret),
                _  => None
            }
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
