use io::IO;
use weighted_unionfind::WeightedUnionFind;

fn main() {
    let mut io = IO::new(std::io::stdin(), std::io::stdout());
    let (n, q): (usize, usize) = (io.read(), io.read());
    let mut uf = WeightedUnionFind::<i32>::new(n);
    for _ in 0..q {
        let c: usize = io.read();
        match c {
            0 => {
                let (x, y, z): (usize, usize, i32) = (io.read(), io.read(), io.read());
                uf.merge(x, y, z);
            },
            _ => {
                let (x, y): (usize, usize) = (io.read(), io.read());
                if uf.is_same(x, y) {
                    let d = uf.diff(x, y);
                    io.write(format!("{}\n", d));
                } else {
                    io.write(format!("{}\n", "?"));
                }
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
    impl<T: Magma> Associative for T {}

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
        ($($t: tt)*) => {$(
            impl Zero for $t {
                fn zero() -> Self { 0 }
                fn is_zero(&self) -> bool { self == &0 }
            }
            impl One for $t {
                fn one() -> Self { 1 }
                fn is_one(&self) -> bool { self == &1 }
            }
            impl Bounded for $t {
                fn min_value() -> Self { std::$t::MIN }
                fn max_value() -> Self { std::$t::MAX }
            }
        )*};
    }
    macro_rules! signed_int_primitives {
        ($($t: tt)*) => {$(
            impl Signed for $t {
                fn abs(&self) -> Self { if self >= &0 { *self } else { -self } }
                fn abs_sub(&self, other: &Self) -> Self { if self >= other { self - other } else { other - self } }
                fn is_positive(&self) -> bool { self > &0 }
                fn is_negative(&self) -> bool { self < &0 }
            }
        )*};
    }
    macro_rules! unsigned_int_primitives {
        ($($t: tt)*) => {$(
            impl Unsigned for $t {}
        )*};
    }
    macro_rules! floating_primitives {
        ($($t: tt)*) => {$(
            impl Zero for $t {
                fn zero() -> Self { 0.0 }
                fn is_zero(&self) -> bool { self == &0.0 }
            }
            impl One for $t {
                fn one() -> Self { 1.0 }
                fn is_one(&self) -> bool { self == &1.0 }
            }
            impl Signed for $t {
                fn abs(&self) -> Self { if self >= &0.0 { *self } else { -self } }
                fn abs_sub(&self, other: &Self) -> Self { (self - other).abs() }
                fn is_positive(&self) -> bool { self > &0.0 }
                fn is_negative(&self) -> bool { self < &0.0 }
            }
            impl Bounded for $t {
                fn min_value() -> Self { std::$t::MIN }
                fn max_value() -> Self { std::$t::MAX }
            }
        )*};
    }

    integer_primitives!(u128 u64 u32 u16 u8 usize i128 i64 i32 i16 i8 isize);
    signed_int_primitives!(i128 i64 i32 i16 i8 isize);
    unsigned_int_primitives!(u128 u64 u32 u16 u8 usize);
    floating_primitives!(f32 f64);
}

pub mod weighted_unionfind {
    use super::num_trait::*;
    use std::mem::swap;

    #[derive(Debug)]
    pub struct WeightedUnionFind<T> {
        pub parent: Vec<usize>,
        pub rank: Vec<usize>,
        pub weights: Vec<T>,
    }

    impl<T> WeightedUnionFind<T>
    where
        T: AbelGroup
    {
        pub fn new(n: usize) -> Self {
            let parent = (0..n).collect();
            let rank = vec![0; n];
            let weights = vec![T::zero(); n];
            Self { parent, rank, weights }
        }

        pub fn root(&mut self, x: usize) -> usize {
            if self.parent[x] == x {
                x
            } else {
                let r = self.root(self.parent[x]);
                self.weights[x] = self.weights[x] + self.weights[self.parent[x]];
                self.parent[x] = r;
                self.parent[x]
            }
        }

        pub fn weight(&mut self, x: usize) -> T {
            self.root(x);
            self.weights[x]
        }

        pub fn is_same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }

        pub fn merge(&mut self, mut x: usize, mut y: usize, mut w: T) -> bool {
            w = w + self.weight(x);
            w = w + -self.weight(y);
            x = self.root(x);
            y = self.root(y);
            if x == y { return false }
            if self.rank[x] < self.rank[y] {
                swap(&mut x, &mut y);
                w = -w;
            }
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
            self.parent[y] = x;
            self.weights[y] = w;
            true
        }

        pub fn diff(&mut self, x: usize, y: usize) -> T {
            self.weight(y) + -self.weight(x)
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
