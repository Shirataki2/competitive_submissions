#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;
use fenwick_tree::*;


#[fastout]
fn main() {
    input!(n: usize, mut a: [i64; n]);
    let mut b = FenwickTree::new(2 * n, |&x, &y| x + y);
    let mut t: i64 = 0;
    for i in (0..n).rev() {
        t += b.sum(a[i] as usize);
        b.add(a[i] as usize, 1);
    }
    println!("{}", t);
    for k in 0..n-1 {
        t += n as i64 - (a[k] << 1) - 1;
        println!("{}", t);
    }
}

pub mod fenwick_tree {
    use super::num_trait::*;

    pub struct FenwickTree<T> {
        pub data: Vec<T>,
        f: fn(&T, &T) -> T,
    }

    impl<T: Monoid> FenwickTree<T> {
        pub fn new(size: usize, f: fn(&T, &T) -> T) -> Self {
            let data = vec![T::zero(); size+1];
            Self { data, f }
        }

        pub fn add(&mut self, k: usize, v: T) {
            let mut k = k as isize;
            k += 1;
            while k < self.data.len() as isize {
                self.data[k as usize] = (self.f)(&self.data[k as usize], &v);
                k += k & -k;
            }
        }

        pub fn sum(&self, k: usize) -> T {
            let mut ret = T::zero();
            let mut k = k as isize;
            k += 1;
            while k > 0 {
                ret = (self.f)(&ret, &self.data[k as usize]);
                k -= k & -k;
            }
            ret
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