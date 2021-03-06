use io::IO;
use graph::Graph;
use strongly_connected_components::StronglyConnectedComponents;

fn main() {
    let mut io = IO::new(std::io::stdin(), std::io::stdout());
    let (v, e): (usize, usize) = (io.read(), io.read());
    let mut g = Graph::new(v);
    for _ in 0..e {
        let (s, t): (usize, usize) = (io.read(), io.read());
        g.add_edge(s, t, 1);
    }
    let mut scc = StronglyConnectedComponents::new(&g);
    scc.build();
    let q: usize = io.read();
    for _ in 0..q {
        let (a, b): (usize, usize) = (io.read(), io.read());
        io.write(format!("{}\n", if scc.same(a, b) { "1" } else { "0" }));
    }
}

pub mod strongly_connected_components {
    use super::graph::*;
    use super::num_trait::*;

    #[derive(Debug)]
    pub struct StronglyConnectedComponents<'a, T> {
        graph: &'a Graph<T>,
        rgraph: Graph<T>,
        visited: Vec<bool>,
        cmp: Vec<isize>,
        ord: Vec<usize>,
        components: Vec<Vec<usize>>,
    }

    impl<'a, T> StronglyConnectedComponents<'a, T>
    where
        T: AbelGroup + Eq + Ord
    {
        pub fn new(graph: &'a Graph<T>) -> Self {
            let n = graph.0.len();
            let mut rgraph = Graph::<T>::new(n);
            for v in 0..n {
                for edge in graph[v].iter() {
                    rgraph[edge.to].push(Edge { to: v, cost: edge.cost })
                }
            }
            let visited = vec![false; n];
            let cmp = vec![-1; n];
            let ord = vec![];
            let components = vec![];
            Self { graph, rgraph, visited, cmp, ord, components }
        }

        pub fn same(&self, s: usize, t: usize) -> bool {
            self.cmp[s] == self.cmp[t]
        }

        fn dfs(&mut self, s: usize) {
            if self.visited[s] { return }
            self.visited[s] = true;
            for edge in self.graph[s].iter() {
                self.dfs(edge.to);
            }
            self.ord.push(s);
        }

        fn rdfs(&mut self, s: usize, ctr: isize) {
            if self.cmp[s] != -1 { return }
            self.cmp[s] = ctr;
            if self.components.len() <= ctr as usize {
                self.components.push(Vec::new());
            }
            self.components[ctr as usize].push(s);
            for edge in self.rgraph[s].clone().iter() {
                self.rdfs(edge.to, ctr);
            }
        }

        pub fn build(&mut self) {
            for i in 0..self.graph.0.len() {
                self.dfs(i);
            }
            self.ord.reverse();
            let mut ctr = 0;
            for v in self.ord.clone().iter() {
                if self.cmp[*v] != -1 { continue }
                self.rdfs(*v, ctr);
                if self.components.len() > 1 {
                    self.components[ctr as usize].reverse();
                }
                ctr += 1;
            }
        }
    }
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
