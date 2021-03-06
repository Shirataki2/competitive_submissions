use io::IO;
use dinic::*;

fn main() {
    let mut io = IO::new(std::io::stdin(), std::io::stdout());
    let (v, e): (usize, usize) = (io.read(), io.read());
    let mut g = Dinic::new(v);
    for _ in 0..e {
        let (u, v, c): (usize, usize, i64) = (io.read(), io.read(), io.read());
        g.add_flow(u, v, c);
    }
    io.write(format!("{}\n", g.max_flow(0, v-1)));
}

pub mod dinic {
    use super::num_trait::*;
    use std::cmp::*;
    use std::collections::VecDeque;

    #[derive(Debug, Clone, Copy)]
    struct Edge<T> {
        to: usize,
        cap: T,
        rev: i64,
    }

    #[derive(Debug)]
    pub struct Dinic<T> {
        graph: Vec<Vec<Edge<T>>>,
        min_cost: Vec<T>,
        iter: Vec<usize>,
    }

    impl<T: Ring + Bounded + Eq + Ord> Dinic<T> {
        pub fn new(size: usize) -> Self {
            let graph = vec![vec![]; size];
            let min_cost = vec![];
            let iter = vec![];
            Self { graph, min_cost, iter }
        }

        pub fn add_flow(&mut self, from: usize, to: usize, cap: T) {
            let len_to = self.graph[to].len() as i64;
            let len_from = self.graph[from].len() as i64;
            self.graph[from].push(Edge { to, cap, rev: len_to });
            self.graph[to].push(Edge { to: from, cap: T::zero(), rev: len_from });
        }

        fn bfs(&mut self, s: usize, t: usize) -> bool {
            let n = self.graph.len();
            self.min_cost = vec![T::max_value(); n];
            let mut q = VecDeque::new();
            self.min_cost[s] = T::zero();
            q.push_back(s);
            while !q.is_empty() {
                let p = q.pop_front().unwrap();
                for edge in self.graph[p].iter() {
                    if edge.cap > T::zero() && self.min_cost[edge.to] == T::max_value() {
                        self.min_cost[edge.to] = self.min_cost[p] + T::one();
                        q.push_back(edge.to);
                    }
                }
            }
            self.min_cost[t] != T::max_value()
        }

        fn dfs(&mut self, idx: usize, t: usize, flow: T) -> T {
            if idx == t { return flow }
            let n = self.graph[idx].len();
            while self.iter[idx] < n {
                let edge = self.graph[idx][self.iter[idx]];
                if edge.cap > T::zero() && self.min_cost[idx] < self.min_cost[edge.to] {
                    let d = self.dfs(edge.to, t, min(flow, edge.cap));
                    if d > T::zero() {
                        self.graph[idx][self.iter[idx]].cap -= d;
                        self.graph[edge.to][edge.rev as usize].cap += d;
                        return d;
                    }
                }
                self.iter[idx] += 1;
            }
            T::zero()
        }

        pub fn max_flow(&mut self, s: usize, t: usize) -> T {
            let mut flow = T::zero();
            while self.bfs(s, t) {
                self.iter = vec![0; self.graph.len()];
                loop {
                    let f = self.dfs(s, t, T::max_value());
                    if f <= T::zero() { break }
                    flow += f;
                }
            }
            flow
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

    pub trait Group: Monoid + Neg<Output=Self> + Sub<Output=Self> {}
    impl<T: Monoid + Neg<Output=Self> + SubAssign + Sub<Output=Self>> Group for T {}

    pub trait AbelGroup: ComMonoid + Group + SubAssign {}
    impl<T: Group + ComMonoid + SubAssign> AbelGroup for T {}

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
