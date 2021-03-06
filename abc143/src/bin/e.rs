#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;
use warshall_floyd::*;


#[fastout]
fn main() {
    input!(n: usize, m: usize, l: i64);
    let mut g = WarshallFloyd::new(n);
    for _ in 0..m {
        input!(a: usize, b: usize, c: i64);
        g.add_edge_undirected(a-1, b-1, c);
    }
    g.build_graph();
    let mut h = WarshallFloyd::new(n);
    for i in 0..n-1 {
        for j in i+1..n {
            if g[i][j] <= l {
                h.add_edge_undirected(i, j, 1);            
            }
        }
    }
    h.build_graph();
    input!(q: usize);
    for _ in 0..q {
        input!(mut c: usize, mut d: usize);
        c -= 1; d -= 1;
        println!("{}", if h[c][d] > 100000 { -1 } else { h[c][d] - 1 });
    }
}

pub mod warshall_floyd {
    use super::{min, chmin};
    use super::num_trait::*;
    use std::ops::*;

    #[derive(Debug)]
    pub struct WarshallFloyd<T>(Vec<Vec<T>>);

    impl<T: AbelGroup + Bounded + Signed> WarshallFloyd<T> {
        #[allow(clippy::needless_range_loop)]
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
            (0..n).any(|i| self[i][i] < T::zero())
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
        fn is_positive(&self) -> bool;
        fn is_negative(&self) -> bool;
    }

    pub trait Unsigned: Sized {}

    pub trait Bounded: Sized {
        fn min_value() -> Self;
        fn max_value() -> Self;
    }

    pub trait BaseNumber {}
    pub trait BaseInteger: BaseNumber {}

    macro_rules! fn_float {
        ($($f: ident)*) => {
            $(fn $f(self) -> Self;)*
        };
    }

    macro_rules! impl_float {
        ($($f: ident)*) => {
            $(
                #[allow(unconditional_recursion)]
                fn $f(self) -> Self { self.$f() }
            )*
        };
    }

    pub trait BaseFloating: BaseNumber + Field + Rem<Output=Self> + RemAssign {
        fn_float!(
            float ceil round trunc fract abs signum sqrt
            exp exp2 ln log2 log10 cbrt sin cos tan
            asin acos atan exp_m1 ln_1p sinh cosh tanh
            asinh acosh atanh recip to_degrees to_radians
        );

        fn sin_cos(&self) -> (Self, Self);
        fn atan2(&self, rhs: Self) -> Self;

        fn eps() -> Self;
        fn pi() -> Self;
        fn pi_deg() -> Self;
        fn tau() -> Self;
        fn tau_deg() -> Self;

        fn approx_eq(self, rhs: Self) -> bool {
            if self == Self::zero() {
                rhs.abs() < Self::eps()
            } else if rhs == Self::zero() {
                self.abs() < Self::eps()
            } else {
                let (x, y) = (self.abs(), rhs.abs());
                (x - y).abs() / (if x < y { x } else { y }) < Self::eps()
            }
        }
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
            impl BaseNumber for $name {}
            impl BaseInteger for $name {}
        )*};
    }
    macro_rules! signed_int_primitives {
        ($($name: tt)*) => {$(
            impl Signed for $name {
                fn abs(&self) -> Self { if self >= &0 { *self } else { -self } }
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
                fn is_zero(&self) -> bool { -1e-6 < *self && 1e-6 > *self }
            }
            impl One for $name {
                fn one() -> Self { 1.0 }
                fn is_one(&self) -> bool { 1.0 - 1e-6 < *self && 1.0 + 1e-6 > *self }
            }
            impl Signed for $name {
                fn abs(&self) -> Self { if self >= &0.0 { *self } else { -self } }
                fn is_positive(&self) -> bool { self > &0.0 }
                fn is_negative(&self) -> bool { self < &0.0 }
            }
            impl Bounded for $name {
                fn min_value() -> Self { std::$name::MIN }
                fn max_value() -> Self { std::$name::MAX }
            }
            impl Associative for $name {}
            impl BaseNumber for $name {}
            impl BaseFloating for $name {
                impl_float!(
                    float ceil round trunc fract abs signum sqrt
                    exp exp2 ln log2 log10 cbrt sin cos tan
                    asin acos atan exp_m1 ln_1p sinh cosh tanh
                    asinh acosh atanh recip to_degrees to_radians
                );

                #[allow(unconditional_recursion)]
                fn sin_cos(&self) -> (Self, Self) {
                    self.sin_cos()
                }

                #[allow(unconditional_recursion)]
                fn atan2(&self, rhs: Self) -> Self { self.atan2(rhs) }
                fn eps() -> Self { std::$name::EPSILON }
                fn pi() -> Self { std::$name::consts::PI }
                fn pi_deg() -> Self { 180.0 }
                fn tau() -> Self { std::$name::consts::PI * 2.0 }
                fn tau_deg() -> Self { 360.0 }
            }
        )*};
    }

    integer_primitives!(u128 u64 u32 u16 u8 usize i128 i64 i32 i16 i8 isize);
    signed_int_primitives!(i128 i64 i32 i16 i8 isize);
    unsigned_int_primitives!(u128 u64 u32 u16 u8 usize);
    floating_primitives!(f32 f64);
}
