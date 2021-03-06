use io::IO;
use warshall_floyd::WarshallFloyd;

fn main() {
    let mut io = IO::new(std::io::stdin(), std::io::stdout());
    let (n, m): (usize, usize) = (io.read(), io.read());
    let mut wf = WarshallFloyd::new(n);
    for _ in 0..m {
        let (s, t, d): (usize, usize, i64) = (io.read(), io.read(), io.read());
        wf[s][t] = d;
    }
    wf.build_graph();
    if !wf.has_negative_cycle() {
        for i in 0..n {
            let d = &wf[i];
            let l = d.iter().map(|di| {
                if di != &std::i64::MAX {
                    format!("{}", di)
                } else {
                    format!("INF")
                }
            }).collect::<Vec<_>>().join(" ");
            io.write(format!("{}\n", l));
        }
    } else {
        io.write(format!("NEGATIVE CYCLE\n"));
    }
}

#[macro_export]
macro_rules! min {
    ($a: expr) => { $a };
    ($a: expr, $b: expr) => { std::cmp::min($a, $b) };
    ($a: expr, $($rest: expr),+) => { std::cmp::min($a, min!($($rest),+)) };
}

#[macro_export]
macro_rules! chmin {
    ($a: expr, $($rest: expr),+) => {{
        let cmp_min = min!($($rest),+);
        if $a > cmp_min { $a = cmp_min; true } else { false }
    }};
}

pub mod warshall_floyd {
    use super::{min, chmin};
    use super::num_trait::*;
    use std::ops::*;

    #[derive(Debug)]
    pub struct WarshallFloyd<T>(Vec<Vec<T>>);

    impl<T: AbelGroup + Bounded + Signed> WarshallFloyd<T> {
        pub fn new(n: usize) -> Self {
            let mut d = vec![vec![T::max_value(); n]; n];
            for i in 0..n {
                d[i][i] = T::zero();
            }
            Self(d)
        }

        pub fn add_edge(&mut self, from: usize, to: usize, cost: T) {
            self[from][to] = cost;
        }

        pub fn add_edge_undirected(&mut self, from: usize, to: usize, cost: T) {
            self[from][to] = cost;
            self[to][from] = cost;
        }

        pub fn build_graph(&mut self) {
            let n = self.0.len();
            for k in 0..n {
                for i in 0..n {
                    for j in 0..n {
                        if self[i][k] != T::max_value() && self[k][j] != T::max_value() {
                            chmin!(self[i][j], self[i][k] + self[k][j]);
                        }
                    }
                }
            }
        }

        pub fn has_negative_cycle(&self) -> bool {
            let n = self.0.len();
            (0..n).fold(false, |x, i| x || self[i][i] < T::zero())
        }
    }

    impl<T> Index<usize> for WarshallFloyd<T> {
        type Output = Vec<T>;
        fn index(&self, index: usize) -> &Self::Output {
            &self.0[index]
        }
    }

    impl<T> IndexMut<usize> for WarshallFloyd<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.0[index]
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