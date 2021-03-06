use io::IO;
use kruskal::Kruskal;

fn main() {
    let mut io = IO::new(std::io::stdin(), std::io::stdout());
    let (n, m): (usize, usize) = (io.read(), io.read());
    let mut kr = Kruskal::new(n);
    for _ in 0..m {
        let (s, t, d): (usize, usize, i64) = (io.read(), io.read(), io.read());
        kr.add_edge_undirected(s, t, d);
    }
    io.write(format!("{}\n", kr.build()))
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

pub mod kruskal {
    use super::num_trait::*;
    use super::unionfind::UnionFind;
    use std::cmp::*;

    pub struct Edge<T: AbelGroup + Eq + Ord> {
        from: usize,
        to: usize,
        cost: T,
    }

    impl<T: AbelGroup + Eq + Ord> PartialEq for Edge<T> {
        fn eq(&self, other: &Self) -> bool {
            self.cost.eq(&other.cost)
        }
    }
    impl<T: AbelGroup + Eq + Ord> PartialOrd for Edge<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.cost.partial_cmp(&other.cost)
        }
    }
    impl<T: AbelGroup + Eq + Ord> Eq for Edge<T> {}
    impl<T: AbelGroup + Eq + Ord> Ord for Edge<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.cost.cmp(&other.cost)
        }
    }

    pub struct Kruskal<T: AbelGroup + Eq + Ord> {
        uft: UnionFind,
        edges: Vec<Edge<T>>,
    }

    impl<T: AbelGroup + Eq + Ord> Kruskal<T> {
        pub fn new(size: usize) -> Self {
            let uft = UnionFind::new(size);
            let edges = vec![];
            Self { uft, edges }
        }

        pub fn add_edge(&mut self, from: usize, to: usize, cost: T) {
            self.edges.push(Edge { from, to, cost });
        }

        pub fn add_edge_undirected(&mut self, from: usize, to: usize, cost: T) {
            self.edges.push(Edge { from, to, cost });
            self.edges.push(Edge { to, from, cost });
        }

        pub fn build(&mut self) -> T {
            self.edges.sort();
            let mut weight_sum = T::zero();
            for edge in self.edges.iter() {
                if !self.uft.is_same(edge.from, edge.to) {
                    self.uft.unite(edge.from, edge.to);
                    weight_sum += edge.cost;
                }
            }
            weight_sum
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
