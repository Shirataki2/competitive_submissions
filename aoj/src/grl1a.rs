use io::IO;
use graph::Graph;
use dijkstra::Dijkstra;

fn main() {
    let mut io = IO::new(std::io::stdin(), std::io::stdout());
    let (n, m, s): (usize, usize, usize) = (io.read(), io.read(), io.read());
    let mut g = Graph::new(n);
    for _ in 0..m {
        let (s, t, d): (usize, usize, i64) = (io.read(), io.read(), io.read());
        g.add_edge(s, t, d);
    }
    let mut g = Dijkstra::new(&g);
    g.build_graph(s);
    g.dists.iter().for_each(|&dist| {
        if dist == std::i64::MAX {
            io.write(format!("INF\n"));
        } else {
            io.write(format!("{}\n", dist));
        }
    })
}

pub mod num_trait {
    use std::ops::*;

    /// Additive identity
    pub trait Zero: Sized {
        fn zero() -> Self;
        fn is_zero(&self) -> bool;
    }

    /// Multiplicative identity
    pub trait One: Sized {
        fn one() -> Self;
        fn is_one(&self) -> bool;
    }

    pub trait Signed: Sized {
        fn abs(&self) -> Self;
        fn abs_sub(&self, other: &Self) -> Self;
        fn is_positive(&self) -> bool;
        fn is_negative(&self) -> bool;
    }

    pub trait Unsigned: Sized {}

    pub trait Bounded: Sized {
        fn min_value() -> Self;
        fn max_value() -> Self;
    }

    pub trait Elem: Sized + Copy + Clone + PartialEq {}
    impl<T: Sized + Clone + Copy + PartialEq> Elem for T {}

    pub trait Magma: Elem + Add<Output=Self> {}
    impl<T: Elem + Add<Output=Self>> Magma for T {}

    pub trait Associative: Magma {}

    pub trait SemiGroup: Magma + Associative {}
    impl<T: Magma + Associative> SemiGroup for T {}

    pub trait Monoid: SemiGroup + Zero {}
    impl<T: SemiGroup + Zero> Monoid for T {}

    pub trait ComMonoid: Monoid + AddAssign + PartialOrd {}
    impl<T: Monoid + AddAssign + PartialOrd> ComMonoid for T {}

    pub trait Group: Monoid + Neg<Output=Self> {}
    impl<T: Monoid + Neg<Output=Self>> Group for T {}

    pub trait AbelGroup: ComMonoid + Group {}
    impl<T: Group + ComMonoid> AbelGroup for T {}

    pub trait SemiRing: ComMonoid + Mul<Output=Self> + One {}
    impl<T: ComMonoid + Mul<Output=Self> + One> SemiRing for T {}

    pub trait Ring: AbelGroup + SemiRing {}
    impl<T: AbelGroup + SemiRing> Ring for T {}

    pub trait ComRing: Ring + MulAssign {}
    impl<T: Ring + MulAssign> ComRing for T {}

    pub trait Field: ComRing + Div<Output=Self> + DivAssign {}
    impl<T: ComRing + Div<Output=Self> + DivAssign> Field for T {}

    macro_rules! integer_primitives {
        ($($name: tt)*) => {$(
            impl Zero for $name {
                fn zero() -> Self { 0 }
                fn is_zero(&self) -> bool { self == &0 }
            }
            impl One for $name {
                fn one() -> Self { 1 }
                fn is_one(&self) -> bool { self == &1 }
            }
            impl Bounded for $name {
                fn min_value() -> Self { std::$name::MIN }
                fn max_value() -> Self { std::$name::MAX }
            }
            impl Associative for $name {}
        )*};
    }
    macro_rules! signed_int_primitives {
        ($($name: tt)*) => {$(
            impl Signed for $name {
                fn abs(&self) -> Self { if self >= &0 { *self } else { -self } }
                fn abs_sub(&self, other: &Self) -> Self { if self >= other { self - other } else { other - self } }
                fn is_positive(&self) -> bool { self > &0 }
                fn is_negative(&self) -> bool { self < &0 }
            }
        )*};
    }
    macro_rules! unsigned_int_primitives {
        ($($name: tt)*) => {$(
            impl Unsigned for $name {}
        )*};
    }
    macro_rules! floating_primitives {
        ($($name: tt)*) => {$(
            impl Zero for $name {
                fn zero() -> Self { 0.0 }
                fn is_zero(&self) -> bool { self == &0.0 }
            }
            impl One for $name {
                fn one() -> Self { 1.0 }
                fn is_one(&self) -> bool { self == &1.0 }
            }
            impl Signed for $name {
                fn abs(&self) -> Self { if self >= &0.0 { *self } else { -self } }
                fn abs_sub(&self, other: &Self) -> Self { (self - other).abs() }
                fn is_positive(&self) -> bool { self > &0.0 }
                fn is_negative(&self) -> bool { self < &0.0 }
            }
            impl Bounded for $name {
                fn min_value() -> Self { std::$name::MIN }
                fn max_value() -> Self { std::$name::MAX }
            }
            impl Associative for $name {}
        )*};
    }

    integer_primitives!(u128 u64 u32 u16 u8 usize i128 i64 i32 i16 i8 isize);
    signed_int_primitives!(i128 i64 i32 i16 i8 isize);
    unsigned_int_primitives!(u128 u64 u32 u16 u8 usize);
    floating_primitives!(f32 f64);
}

pub mod graph {
    use super::num_trait::*;
    use std::cmp::*;
    use std::ops::*;

    #[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
    pub struct Edge<T> {
        pub to: usize,
        pub cost: T,
    }
    impl<T: AbelGroup + Eq + Ord> Edge<T> {
        pub fn new(to: usize, cost: T) -> Self {
            Self { to, cost }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Graph<T>(pub Vec<Vec<Edge<T>>>);

    impl<T: AbelGroup + Eq + Ord> Graph<T> {
        pub fn new(n: usize) -> Self {
            let graph = vec![vec![]; n];
            Self(graph)
        }

        pub fn add_edge(&mut self, from: usize, to: usize, cost: T) {
            self[from].push(Edge::new(to, cost));
        }

        pub fn add_edge_undirected(&mut self, from: usize, to: usize, cost: T) {
            self[from].push(Edge::new(to, cost));
            self[to].push(Edge::new(from, cost));
        }
    }

    impl<T> Index<usize> for Graph<T> {
        type Output = Vec<Edge<T>>;
        fn index(&self, index: usize) -> &Self::Output {
            &self.0[index]
        }
    }

    impl<T> IndexMut<usize> for Graph<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.0[index]
        }
    }
}

pub mod dijkstra {
    use super::num_trait::*;
    use super::graph::*;
    use std::cmp::*;
    use std::collections::BinaryHeap;

    #[derive(Debug, Clone)]
    pub struct Dijkstra<'a, T> {
        graph: &'a Graph<T>,
        pub dists: Vec<T>,
        backs: Vec<isize>,
    }

    impl<'a, T: AbelGroup + Bounded + Eq + Ord> Dijkstra<'a, T> {
        pub fn new(graph: &'a Graph<T>) -> Self {
            let n = graph.0.len();
            let dists = vec![T::max_value(); n];
            let backs = vec![-1; n];
            Self { graph, dists, backs }
        }

        pub fn build_graph(&mut self, s: usize) {
            let mut pq = BinaryHeap::new();
            self.dists[s] = T::zero();
            pq.push(Edge::new(s, self.dists[s]));
            while !pq.is_empty() {
                let p = pq.pop().unwrap();
                let v = p.to;
                if self.dists[v] < p.cost { continue; }
                for edge in self.graph[v].iter() {
                    if self.dists[edge.to] > self.dists[v] + edge.cost {
                        self.dists[edge.to] = self.dists[v] + edge.cost;
                        self.backs[edge.to] = v as isize;
                        pq.push(Edge::new(edge.to, self.dists[edge.to]))
                    }
                }
            }
        }

        pub fn restore(&self, mut to: isize) -> Vec<isize> {
            let mut path = vec![];
            if self.backs[to as usize] < 0 {
                path
            } else {
                while to > 0 {
                    path.push(to);
                    to = self.backs[to as usize];
                }
                path.reverse();
                path
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