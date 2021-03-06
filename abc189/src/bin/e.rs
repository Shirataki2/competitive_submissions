#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;
use linalg::*;

#[fastout]
fn main() {
    input!(n: usize);
    let mut points: Vec<Vector3d<i64>> = Vec::with_capacity(n);
    for _ in 0..n {
        input!(x: i64, y: i64);
        points.push(matrix![x, y, 1]);
    }
    let mut affine = Matrix3x3::<i64>::eye();
    let mut hist = vec![affine];
    input!(m: usize);
    for _ in 0..m {
        input!(op: usize);
        let mat: Matrix3x3<i64> = match op {
            1 => matrix![0, 1, 0; -1, 0, 0; 0, 0, 1],
            2 => matrix![0, -1, 0; 1, 0, 0; 0, 0, 1],
            3 => {
                input!(p: i64);
                matrix![-1, 0, 2*p; 0, 1, 0; 0, 0, 1]
            }
            4 => {
                input!(p: i64);
                matrix![1, 0, 0; 0, -1, 2*p; 0, 0, 1]
            }
            _ => unreachable!()
        };
        affine = mat * affine;
        hist.push(affine);
    }
    input!(q: usize);
    for _ in 0..q {
        input!(a: usize, b: usize);
        let coord = hist[a] * points[b-1];
        println!("{} {}", coord[0], coord[1]);
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

pub mod linalg {
    use std::ops::*;
    use std::fmt;
    use super::num_trait::*;

    pub trait Array
    where
        Self: Index<usize, Output=<Self as Array>::Elem>,
        Self: IndexMut<usize, Output=<Self as Array>::Elem>
    {
        type Elem: Copy;

        fn len() -> usize;

        fn dim(&self) -> usize;

        #[inline]
        fn as_ptr(&self) -> *const Self::Elem { &self[0] }

        #[inline]
        fn as_mut_ptr(&mut self) -> *mut Self::Elem { &mut self[0] }

        #[inline]
        fn swap(&mut self, i: usize, j: usize) {
            unsafe { std::ptr::swap(&mut self[i], &mut self[j]) }
        }

        fn sum(&self) -> Self::Elem where Self::Elem: Magma;
    }

    pub trait VectorSpace: Copy + Clone
    where
        Self: Zero + Add<Self, Output=Self> + Sub<Self, Output=Self>,
        Self: Mul<<Self as VectorSpace>::Scalar, Output=Self>,
        Self: Div<<Self as VectorSpace>::Scalar, Output=Self>,
    {
        type Scalar: Copy;

        #[inline]
        fn lerp(self, other: Self, t: Self::Scalar) -> Self {
            self + (other - self) * t
        }
    }

    pub trait MetricSpace: Sized {
        type Metric;
        fn distance2(self, other: Self) -> Self::Metric;

        /// The distance between two values.
        fn distance(self, other: Self) -> Self::Metric
        where
            Self::Metric: BaseFloating,
        {
            BaseFloating::sqrt(Self::distance2(self, other))
        }
    }

    pub trait InnerSpace: VectorSpace
    where
        Self: MetricSpace<Metric=<Self as VectorSpace>::Scalar>
    {
        fn dot(self, other: Self) -> Self::Scalar;

        #[inline]
        fn magnitude2(self) -> Self::Scalar {
            self.dot(self)
        }

        #[inline]
        fn magnitude(self) -> Self::Scalar
        where
            Self::Scalar: BaseFloating
        {
            BaseFloating::sqrt(self.magnitude2())
        }

        #[inline]
        fn normalized(self) -> Self
        where
            Self::Scalar: BaseFloating + Field
        {
            self * (Self::Scalar::one() / self.magnitude())
        }
    }

    pub trait EuclideanSpace: Copy + Clone
    where
        Self: Array<Elem=<Self as EuclideanSpace>::Scalar>,
        Self: Add<<Self as EuclideanSpace>::Diff, Output=Self>,
        Self: Sub<<Self as EuclideanSpace>::Diff, Output=Self>,
        Self: Sub<Self, Output=<Self as EuclideanSpace>::Diff>,
        Self: Mul<<Self as EuclideanSpace>::Scalar, Output=Self>,
        Self: Div<<Self as EuclideanSpace>::Scalar, Output=Self>,
    {
        type Scalar: BaseNumber + Copy;
        type Diff: VectorSpace<Scalar=Self::Scalar>;

        fn origin(self) -> Self;
        fn from_vector(v: Self::Diff) -> Self;
        fn to_vector(self) -> Self::Diff;
    }

    pub trait Matrix: VectorSpace
    where
        Self::Scalar: Field,
        Self: Index<usize, Output=<Self as Matrix>::Column>,
        Self: IndexMut<usize, Output=<Self as Matrix>::Column>,
    {
        type Row: VectorSpace<Scalar=Self::Scalar> + Array<Elem=Self::Scalar>;
        type Column: VectorSpace<Scalar=Self::Scalar> + Array<Elem=Self::Scalar>;
        type Transpose: Matrix<Scalar=Self::Scalar, Row=Self::Column, Column=Self::Row>;

        #[inline]
        fn as_ptr(&self) -> *const Self::Scalar {
            &self[0][0]
        }

        #[inline]
        fn as_mut_ptr(&mut self) -> *mut Self::Scalar {
            &mut self[0][0]
        }

        fn len_row() -> usize { Self::Row::len() }

        fn len_col() -> usize { Self::Column::len() }

        fn get_row(self, r: usize) -> Self::Row;

        fn transpose(self) -> Self::Transpose;
    }

    pub trait SqMatrix
    where
        Self::Scalar: Field,
        Self: Matrix<Row=<Self as SqMatrix>::ColumnRow, Column=<Self as SqMatrix>::ColumnRow, Transpose=Self>,
        Self: Mul<Self, Output=Self>
    {
        type ColumnRow: VectorSpace<Scalar=Self::Scalar> + Array<Elem=Self::Scalar>;
        fn det(&self) -> Self::Scalar;
        fn inv(&self) -> Option<Self>;

        fn len() -> usize { Self::len_row() }
        fn eye() -> Self {
            let mut res = Self::zero();
            for i in 0..Self::len_row() {
                res[i][i] = Self::Scalar::one();
            }
            res
        }
        fn diag(&self) -> Self::ColumnRow {
            let mut res = Self::ColumnRow::zero();
            for i in 0..Self::len() {
                res[i] = self[i][i];
            }
            res
        }
    }

    impl<T: Field> Vector3d<T> {
        pub fn cross(self, rhs: Self) -> Self {
            Self::new([
                self[1] * rhs[2] - self[2] * rhs[1],
                self[2] * rhs[0] - self[0] * rhs[2],
                self[0] * rhs[1] - self[1] * rhs[0],
            ])
        }
    }

    impl<T: Field> SqMatrix for Matrix2x2<T> {
        type ColumnRow = Vector2d<T>;

        fn det(&self) -> T {
            self[0][0] * self[1][1] - self[0][1] * self[1][0]
        }
        fn inv(&self) -> Option<Self> {
            let det = self.det();
            if T::is_zero(&det) {
                None
            } else {
                Some(Matrix2x2::new([
                    Vector2d::new([ self[1][1], -self[0][1]]),
                    Vector2d::new([-self[1][0],  self[0][0]]),
                ]) / det)
            }
        }
    }

    impl<T: Field> SqMatrix for Matrix3x3<T> {
        type ColumnRow = Vector3d<T>;

        fn det(&self) -> T {
            self[0][0] * (self[1][1] * self[2][2] - self[1][2] * self[2][1])
            - self[1][0] * (self[0][1] * self[2][2] - self[2][1] * self[0][2])
            + self[2][0] * (self[0][1] * self[1][2] - self[1][1] * self[0][2])
        }
        fn inv(&self) -> Option<Self> {
            let det = self.det();
            if T::is_zero(&det) {
                None
            } else {
                Some(Matrix3x3::new([
                    self[1].cross(self[2]) / det,
                    self[2].cross(self[0]) / det,
                    self[0].cross(self[1]) / det,
                ]).transpose())
            }
        }
    }

    impl<T: Field + BaseFloating> SqMatrix for Matrix4x4<T> {
        type ColumnRow = Vector4d<T>;

        fn det(&self) -> T {
            let tmp = unsafe { det_sub_proc_unsafe(self, 1, 2, 3) };
            tmp.dot(Vector4d::new([self[0][0], self[1][0], self[2][0], self[3][0]]))
        }

        fn inv(&self) -> Option<Self> {
            let tmp = unsafe { det_sub_proc_unsafe(self, 1, 2, 3) };
            let det = tmp.dot(Vector4d::new([self[0][0], self[1][0], self[2][0], self[3][0]]));
            if T::is_zero(&det) {
                None
            } else {
                let inv_det = T::one() / det;
                let tmp0 = tmp * inv_det;
                let tmp1 = unsafe { det_sub_proc_unsafe(self, 0, 3, 2) * inv_det };
                let tmp2 = unsafe { det_sub_proc_unsafe(self, 0, 1, 3) * inv_det };
                let tmp3 = unsafe { det_sub_proc_unsafe(self, 0, 2, 1) * inv_det };
                Some(Self::new([tmp0, tmp1, tmp2, tmp3]))
            }
        }
    }

    pub struct VectorIterator<'a, V, T>
    where
        V: VectorSpace<Scalar=T> + Array<Elem=T>,
        T: Copy
    {
        v: &'a V,
        idx: usize,
    }

    impl<'a, V, T: 'a> Iterator for VectorIterator<'a, V, T>
    where
        V: VectorSpace<Scalar=T> + Array<Elem=T>,
        T: Copy
    {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            if self.idx < V::len() {
                let v = &self.v[self.idx];
                self.idx += 1;
                Some(v)
            } else {
                None
            }
        }
    }

    pub struct VectorIntoIterator<V, T>
    where
        V: VectorSpace<Scalar=T> + Array<Elem=T>,
        T: Copy
    {
        v: V,
        idx: usize,
    }

    impl<V, T> Iterator for VectorIntoIterator<V, T>
    where
        V: VectorSpace<Scalar=T> + Array<Elem=T>,
        T: Copy
    {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            if self.idx < V::len() {
                let v = self.v[self.idx];
                self.idx += 1;
                Some(v)
            } else {
                None
            }
        }
    }

    pub struct MatrixIterator<'a, M, T>
    where
        T: Field,
        M: Matrix<Scalar=T>
    {
        m: &'a M,
        row: usize,
        col: usize,
    }

    impl<'a, M, T: 'a> Iterator for MatrixIterator<'a, M, T>
    where
        M: Matrix<Scalar=T>,
        T: Field + Copy
    {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            if self.row < M::Row::len() {
                let v = &self.m[self.row][self.col];
                self.col += 1;
                if self.col == M::Column::len() {
                    self.row += 1;
                    self.col = 0;
                }
                Some(v)
            } else {
                None
            }
        }
    }

    pub struct MatrixIntoIterator<M, T>
    where
        T: Field,
        M: Matrix<Scalar=T>
    {
        m: M,
        row: usize,
        col: usize,
    }

    impl<M, T> Iterator for MatrixIntoIterator<M, T>
    where
        M: Matrix<Scalar=T>,
        T: Field + Copy
    {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            if self.row < M::Row::len() {
                let v = self.m[self.row][self.col];
                self.col += 1;
                if self.col == M::Column::len() {
                    self.row += 1;
                    self.col = 0;
                }
                Some(v)
            } else {
                None
            }
        }
    }

    macro_rules! define_vector {
        ($Name: ident, $dim: expr) => {
            #[derive(Copy, Clone, Debug, PartialEq)]
            pub struct $Name<T>([T; $dim]);

            impl<T> Index<usize> for $Name<T> {
                type Output = T;
                fn index(&self, i: usize) -> &T { &self.0[i] }
            }
            impl<T> IndexMut<usize> for $Name<T> {
                fn index_mut(&mut self, i: usize) -> &mut T { &mut self.0[i] }
            }

            impl<T: Field> IntoIterator for $Name<T> {
                type Item = T;
                type IntoIter = VectorIntoIterator<$Name<T>, T>;

                fn into_iter(self) -> Self::IntoIter {
                    VectorIntoIterator {
                        v: self,
                        idx: 0
                    }
                }
            }

            impl<T: fmt::Display + Copy> fmt::Display for $Name<T>
            where
                Self: Array<Elem=T>
            {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "[")?;
                    for i in 0..Self::len() {
                        write!(f, "{}", self[i])?;
                        if i != Self::len() - 1 {
                            write!(f, ", ")?;
                        }
                    }
                    write!(f, "]")
                }
            }

            impl<T: Monoid> Zero for $Name<T>
            where
                Self: Array<Elem=T>
            {
                fn zero() -> Self {
                    Self([T::zero(); $dim])
                }
                fn is_zero(&self) -> bool {
                    let mut b = true;
                    for i in 0..Self::len() {
                        b &= self[i] == T::zero();
                    }
                    b
                }
            }

            impl<T: Group> Neg for $Name<T>
            where
                Self: Array<Elem=T>,
            {
                type Output = Self;
                fn neg(self) -> Self::Output {
                    let mut res = self;
                    for i in 0..Self::len() {
                        res[i] = -self[i];
                    }
                    res
                }
            }

            impl<T: ComMonoid> AddAssign<Self> for $Name<T>
            where
                Self: Array<Elem=T>,
            {
                fn add_assign(&mut self, rhs: Self) {
                    for i in 0..Self::len() {
                        self[i] += rhs[i];
                    }
                }
            }

            impl<T: AbelGroup> SubAssign<Self> for $Name<T>
            where
                Self: Array<Elem=T>,
            {
                fn sub_assign(&mut self, rhs: Self) {
                    for i in 0..Self::len() {
                        self[i] -= rhs[i];
                    }
                }
            }

            impl<T: Field> MulAssign<<Self as VectorSpace>::Scalar> for $Name<T>
            where
                Self: Array<Elem=T>,
            {
                fn mul_assign(&mut self, rhs: <Self as VectorSpace>::Scalar) {
                    for i in 0..Self::len() {
                        self[i] *= rhs;
                    }
                }
            }

            impl<T: Field> DivAssign<<Self as VectorSpace>::Scalar> for $Name<T>
            where
                Self: Array<Elem=T>,
            {
                fn div_assign(&mut self, rhs: <Self as VectorSpace>::Scalar) {
                    for i in 0..Self::len() {
                        self[i] /= rhs;
                    }
                }
            }

            impl<T: Magma> Add<Self> for $Name<T>
            where
                Self: Array<Elem=T>,
            {
                type Output = Self;
                fn add(self, rhs: Self) -> Self::Output {
                    let mut res = self;
                    for i in 0..Self::len() {
                        res[i] = res[i] + rhs[i];
                    }
                    res
                }
            }

            impl<T: Group> Sub<Self> for $Name<T>
            where
                Self: Array<Elem=T>,
            {
                type Output = Self;
                fn sub(self, rhs: Self) -> Self::Output {
                    let mut res = self;
                    for i in 0..Self::len() {
                        res[i] = res[i] - rhs[i];
                    }
                    res
                }
            }

            impl<T: Field> Mul<<Self as VectorSpace>::Scalar> for $Name<T>
            where
                Self: Array<Elem=T>,
            {
                type Output = Self;
                fn mul(self, rhs: <Self as VectorSpace>::Scalar) -> Self::Output {
                    let mut res = self;
                    for i in 0..Self::len() {
                        res[i] *= rhs;
                    }
                    res
                }
            }

            #[allow(clippy::assign_op_pattern)]
            impl<T: Field> Div<<Self as VectorSpace>::Scalar> for $Name<T>
            where
                Self: Array<Elem=T>,
            {
                type Output = Self;
                fn div(self, rhs: <Self as VectorSpace>::Scalar) -> Self::Output {
                    let mut res = self;
                    for i in 0..Self::len() {
                        res[i] /= rhs;
                    }
                    res
                }
            }

            impl<T> Array for $Name<T>
            where
                T: Monoid
            {
                type Elem = T;
                fn len() -> usize { $dim }
                fn dim(&self) -> usize { $dim }
                fn sum(&self) -> Self::Elem {
                    let mut v = T::zero();
                    for i in 0..Self::len() {
                        v = v + self[i];
                    }
                    v
                }
            }

            impl<T: Field> VectorSpace for $Name<T> {
                type Scalar = T;
            }

            impl<T: Field + BaseFloating> MetricSpace for $Name<T>
            where
                Self: Array<Elem=T>
            {
                type Metric = T;
                fn distance2(self, rhs: Self) -> Self::Metric {
                    let mut res = T::zero();
                    for i in 0..Self::len() {
                        let d = self[i] - rhs[i];
                        res += d * d;
                    }
                    res
                }
            }

            impl<T: Field + BaseFloating> InnerSpace for $Name<T>
            where
                Self: MetricSpace<Metric=T>
            {
                fn dot(self, rhs: Self) -> Self::Scalar {
                    let mut res = T::zero();
                    for i in 0..Self::len() {
                        res += self[i] * rhs[i];
                    }
                    res
                }
            }

            impl<T: Field + BaseFloating> EuclideanSpace for $Name<T>
            where
                Self: VectorSpace<Scalar=T>
            {
                type Scalar = T;
                type Diff = Self;

                fn origin(self) -> Self {
                    Self::zero()
                }

                fn from_vector(v: Self::Diff) -> Self {
                    Self(v.0)
                }

                fn to_vector(self) -> Self::Diff {
                    Self(self.0)
                }
            }

            impl<T: Field> $Name<T> {
                pub fn new(v: [T; $dim]) -> Self {
                    Self(v)
                }

                pub fn iter(&self) -> VectorIterator<Self, T> {
                    VectorIterator {
                        v: self,
                        idx: 0
                    }
                }

                pub fn add_elementwise(self, rhs: Self) -> Self {
                    let mut res = Self::zero();
                    for i in 0..Self::len() {
                        res[i] = self[i] + rhs[i];
                    }
                    res
                }
                pub fn sub_elementwise(self, rhs: Self) -> Self {
                    let mut res = Self::zero();
                    for i in 0..Self::len() {
                        res[i] = self[i] - rhs[i];
                    }
                    res
                }
                pub fn mul_elementwise(self, rhs: Self) -> Self {
                    let mut res = Self::zero();
                    for i in 0..Self::len() {
                        res[i] = self[i] * rhs[i];
                    }
                    res
                }
                pub fn div_elementwise(self, rhs: Self) -> Self {
                    let mut res = Self::zero();
                    for i in 0..Self::len() {
                        res[i] = self[i] / rhs[i];
                    }
                    res
                }
            }
        };
    }

    macro_rules! define_matrix {
        ($Name:ident, $row: expr, $col: expr, $RowVector:ident, $ColVector:ident, $TransMatrix:ident) => {
            #[derive(Copy, Clone, Debug, PartialEq)]
            pub struct $Name<T>([$ColVector<T>; $row]);

            impl<T> Index<usize> for $Name<T> {
                type Output = $ColVector<T>;
                fn index(&self, i: usize) -> &Self::Output { &self.0[i] }
            }
            impl<T> IndexMut<usize> for $Name<T> {
                fn index_mut(&mut self, i: usize) -> &mut Self::Output { &mut self.0[i] }
            }

            impl<T: Field> IntoIterator for $Name<T>
            where
                T: Field + Copy
            {
                type Item = T;
                type IntoIter = MatrixIntoIterator<Self, T>;

                fn into_iter(self) -> Self::IntoIter {
                    MatrixIntoIterator {
                        m: self,
                        row: 0,
                        col: 0,
                    }
                }
            }

            impl<T: Field + fmt::Display, C> fmt::Display for $Name<T>
            where
                Self: Matrix<Scalar=T, Column=C>,
                C: fmt::Display + VectorSpace<Scalar=T> + Array<Elem=T>
            {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "[")?;
                    for i in 0..Self::len_row() {
                        write!(f, "{}", self[i])?;
                        if i != Self::len_row() - 1 {
                            write!(f, ", ")?;
                        }
                    }
                    write!(f, "]")
                }
            }

            impl<T: Monoid> Zero for $Name<T> {
                fn zero() -> $Name<T> {
                    $Name([$ColVector::zero(); $row])
                }
                fn is_zero(&self) -> bool { true }
            }

            impl<T: ComMonoid> AddAssign<Self> for $Name<T>
            {
                fn add_assign(&mut self, rhs: Self) {
                    for i in 0..$row {
                        for j in 0..$col {
                            self[i][j] += rhs[i][j];
                        }
                    }
                }
            }

            impl<T: AbelGroup> SubAssign<Self> for $Name<T>
            {
                fn sub_assign(&mut self, rhs: Self) {
                    for i in 0..$row {
                        for j in 0..$col {
                            self[i][j] -= rhs[i][j];
                        }
                    }
                }
            }

            #[allow(clippy::assign_op_pattern)]
            impl<T: Field> MulAssign<<Self as VectorSpace>::Scalar> for $Name<T>
            {
                fn mul_assign(&mut self, rhs: <Self as VectorSpace>::Scalar) {
                    for i in 0..$row {
                        self[i] *= rhs;
                    }
                }
            }

            #[allow(clippy::assign_op_pattern)]
            impl<T: Field> DivAssign<<Self as VectorSpace>::Scalar> for $Name<T>
            {
                fn div_assign(&mut self, rhs: <Self as VectorSpace>::Scalar) {
                    for i in 0..$row {
                        self[i] /= rhs;
                    }
                }
            }


            impl<T: Magma> Add<Self> for $Name<T>
            {
                type Output = Self;
                fn add(self, rhs: Self) -> Self::Output {
                    let mut res = self;
                    for i in 0..$row {
                        for j in 0..$col {
                            res[i][j] = res[i][j] + rhs[i][j];
                        }
                    }
                    res
                }
            }

            impl<T: Group> Sub<Self> for $Name<T>
            {
                type Output = Self;
                fn sub(self, rhs: Self) -> Self::Output {
                    let mut res = self;
                    for i in 0..$row {
                        res[i] = res[i] - rhs[i];
                    }
                    res
                }
            }

            #[allow(clippy::assign_op_pattern)]
            impl<T: Field> Mul<<Self as VectorSpace>::Scalar> for $Name<T>
            {
                type Output = Self;
                fn mul(self, rhs: <Self as VectorSpace>::Scalar) -> Self::Output {
                    let mut res = self;
                    for i in 0..$row {
                        res[i] = res[i] * rhs;
                    }
                    res
                }
            }

            #[allow(clippy::assign_op_pattern)]
            impl<T: Field> Div<<Self as VectorSpace>::Scalar> for $Name<T>
            {
                type Output = Self;
                fn div(self, rhs: <Self as VectorSpace>::Scalar) -> Self::Output {
                    let mut res = self;
                    for i in 0..$row {
                        res[i] = res[i] / rhs;
                    }
                    res
                }
            }

            impl<T: Field> VectorSpace for $Name<T> {
                type Scalar = T;
            }

            impl<T: Field> $Name<T> {
                pub fn new(v: [$ColVector<T>; $row]) -> Self {
                    Self(v)
                }
            }

            impl<T: Field> From<[$ColVector<T>; $row]> for $Name<T> {
                fn from(v: [$ColVector<T>; $row]) -> Self {
                    Self::new(v)
                }
            }

            #[allow(clippy::transmute_ptr_to_ptr)]
            impl<T: Field> AsRef<[[T; $col]; $row]> for $Name<T> {
                fn as_ref(&self) -> &[[T; $col]; $row] {
                    unsafe { std::mem::transmute(self) }
                }
            }

            #[allow(clippy::transmute_ptr_to_ptr)]
            impl<T: Field> AsMut<[[T; $col]; $row]> for $Name<T> {
                fn as_mut(&mut self) -> &mut [[T; $col]; $row] {
                    unsafe { std::mem::transmute(self) }
                }
            }

            #[allow(clippy::transmute_ptr_to_ptr)]
            impl<T: Field> AsRef<[T; ($col * $row)]> for $Name<T> {
                fn as_ref(&self) -> &[T; ($col * $row)] {
                    unsafe { std::mem::transmute(self) }
                }
            }

            #[allow(clippy::transmute_ptr_to_ptr)]
            impl<T: Field> AsMut<[T; ($col * $row)]> for $Name<T> {
                fn as_mut(&mut self) -> &mut [T; ($col * $row)] {
                    unsafe { std::mem::transmute(self) }
                }
            }

            impl<T: Field> $Name<T> {
                pub fn iter(&self) -> MatrixIterator<Self, T> {
                    MatrixIterator {
                        m: self,
                        row: 0,
                        col: 0,
                    }
                }
            }
        };
    }

    macro_rules! impl_mat_mat_inner_product {
        ($Left:ident, $Right:ident, $Output: ident) => {
            impl<T: Field> Mul<$Right<T>> for $Left<T> {
                type Output = $Output<T>;
                fn mul(self, rhs: $Right<T>) -> $Output<T> {
                    let mut res = $Output::<T>::zero();
                    for i in 0..$Left::<T>::len_row() {
                        for j in 0..$Right::<T>::len_col() {
                            let mut s = T::zero();
                            for k in 0..$Right::<T>::len_row() {
                                s += self[i][k] * rhs[k][j];
                            }
                            res[i][j] = s;
                        }
                    }
                    res
                }
            }
        }
    }

    macro_rules! impl_sqmat_assign_inner_product {
        ($M:ident) => {
            impl<T: Field> MulAssign<$M<T>> for $M<T> {
                fn mul_assign(&mut self, rhs: $M<T>) {
                    let mut res = $M::<T>::zero();
                    for i in 0..$M::<T>::len_row() {
                        for j in 0..$M::<T>::len_col() {
                            let mut s = T::zero();
                            for k in 0..$M::<T>::len_row() {
                                s += self[i][k] * rhs[k][j];
                            }
                            res[i][j] = s;
                        }
                    }
                    *self = res;
                }
            }
        }
    }

    macro_rules! impl_mat_vec_inner_product {
        ($Left:ident, $Right:ident, $Output: ident) => {
            impl<T: Field> Mul<$Right<T>> for $Left<T> {
                type Output = $Output<T>;
                fn mul(self, rhs: $Right<T>) -> $Output<T> {
                    let mut res = $Output::<T>::zero();
                    for i in 0..$Left::<T>::len_row() {
                        let mut s = T::zero();
                        for j in 0..$Right::<T>::len() {
                            s += self[i][j] * rhs[j];
                        }
                        res[i] = s;
                    }
                    res
                }
            }
        }
    }

    macro_rules! impl_vec_vec_inner_product {
        ($V:ident) => {
            impl<T: Field> Mul<$V<T>> for $V<T> {
                type Output = T;
                fn mul(self, rhs: $V<T>) -> T {
                    let mut res = T::zero();
                    for i in 0..$V::<T>::len() {
                        res += self[i] * rhs[i];
                    }
                    res
                }
            }
        }
    }

    define_vector!(Vector2d, 2);
    define_vector!(Vector3d, 3);
    define_vector!(Vector4d, 4);

    define_matrix!(Matrix2x2, 2, 2, Vector2d, Vector2d, Matrix2x2);
    define_matrix!(Matrix2x3, 2, 3, Vector2d, Vector3d, Matrix3x2);
    define_matrix!(Matrix2x4, 2, 4, Vector2d, Vector4d, Matrix4x2);

    define_matrix!(Matrix3x2, 3, 2, Vector3d, Vector2d, Matrix2x3);
    define_matrix!(Matrix3x3, 3, 3, Vector3d, Vector3d, Matrix3x3);
    define_matrix!(Matrix3x4, 3, 4, Vector3d, Vector4d, Matrix4x3);

    define_matrix!(Matrix4x2, 4, 2, Vector4d, Vector2d, Matrix2x4);
    define_matrix!(Matrix4x3, 4, 3, Vector4d, Vector3d, Matrix3x4);
    define_matrix!(Matrix4x4, 4, 4, Vector4d, Vector4d, Matrix4x4);

    impl_vec_vec_inner_product!(Vector2d);
    impl_vec_vec_inner_product!(Vector3d);
    impl_vec_vec_inner_product!(Vector4d);

    impl_mat_vec_inner_product!(Matrix2x2, Vector2d, Vector2d);
    impl_mat_vec_inner_product!(Matrix2x3, Vector3d, Vector2d);
    impl_mat_vec_inner_product!(Matrix2x4, Vector4d, Vector2d);

    impl_mat_vec_inner_product!(Matrix3x2, Vector2d, Vector3d);
    impl_mat_vec_inner_product!(Matrix3x3, Vector3d, Vector3d);
    impl_mat_vec_inner_product!(Matrix3x4, Vector4d, Vector3d);

    impl_mat_vec_inner_product!(Matrix4x2, Vector2d, Vector4d);
    impl_mat_vec_inner_product!(Matrix4x3, Vector3d, Vector4d);
    impl_mat_vec_inner_product!(Matrix4x4, Vector4d, Vector4d);

    impl_sqmat_assign_inner_product!(Matrix2x2);
    impl_sqmat_assign_inner_product!(Matrix3x3);
    impl_sqmat_assign_inner_product!(Matrix4x4);

    impl_mat_mat_inner_product!(Matrix2x2, Matrix2x2, Matrix2x2);
    impl_mat_mat_inner_product!(Matrix2x3, Matrix3x2, Matrix2x2);
    impl_mat_mat_inner_product!(Matrix2x4, Matrix4x2, Matrix2x2);

    impl_mat_mat_inner_product!(Matrix2x2, Matrix2x3, Matrix2x3);
    impl_mat_mat_inner_product!(Matrix2x3, Matrix3x3, Matrix2x3);
    impl_mat_mat_inner_product!(Matrix2x4, Matrix4x3, Matrix2x3);

    impl_mat_mat_inner_product!(Matrix2x2, Matrix2x4, Matrix2x4);
    impl_mat_mat_inner_product!(Matrix2x3, Matrix3x4, Matrix2x4);
    impl_mat_mat_inner_product!(Matrix2x4, Matrix4x4, Matrix2x4);

    impl_mat_mat_inner_product!(Matrix3x2, Matrix2x2, Matrix3x2);
    impl_mat_mat_inner_product!(Matrix3x3, Matrix3x2, Matrix3x2);
    impl_mat_mat_inner_product!(Matrix3x4, Matrix4x2, Matrix3x2);

    impl_mat_mat_inner_product!(Matrix3x2, Matrix2x3, Matrix3x3);
    impl_mat_mat_inner_product!(Matrix3x3, Matrix3x3, Matrix3x3);
    impl_mat_mat_inner_product!(Matrix3x4, Matrix4x3, Matrix3x3);

    impl_mat_mat_inner_product!(Matrix3x2, Matrix2x4, Matrix3x4);
    impl_mat_mat_inner_product!(Matrix3x3, Matrix3x4, Matrix3x4);
    impl_mat_mat_inner_product!(Matrix3x4, Matrix4x4, Matrix3x4);

    impl_mat_mat_inner_product!(Matrix4x2, Matrix2x2, Matrix4x2);
    impl_mat_mat_inner_product!(Matrix4x3, Matrix3x2, Matrix4x2);
    impl_mat_mat_inner_product!(Matrix4x4, Matrix4x2, Matrix4x2);

    impl_mat_mat_inner_product!(Matrix4x2, Matrix2x3, Matrix4x3);
    impl_mat_mat_inner_product!(Matrix4x3, Matrix3x3, Matrix4x3);
    impl_mat_mat_inner_product!(Matrix4x4, Matrix4x3, Matrix4x3);

    impl_mat_mat_inner_product!(Matrix4x2, Matrix2x4, Matrix4x4);
    impl_mat_mat_inner_product!(Matrix4x3, Matrix3x4, Matrix4x4);
    impl_mat_mat_inner_product!(Matrix4x4, Matrix4x4, Matrix4x4);

    impl<T: Field> Matrix for Matrix2x2<T> {
        type Row = Vector2d<T>;
        type Column = Vector2d<T>;
        type Transpose = Matrix2x2<T>;

        fn get_row(self, r: usize) -> Self::Row {
            Vector2d::new([self[r][0], self[r][1]])
        }
        fn transpose(self) -> Self::Transpose {
            Matrix2x2::new([
                Vector2d::new([self[0][0], self[1][0]]),
                Vector2d::new([self[0][1], self[1][1]]),
            ])
        }
    }

    impl<T: Field> Matrix for Matrix2x3<T> {
        type Row = Vector2d<T>;
        type Column = Vector3d<T>;
        type Transpose = Matrix3x2<T>;

        fn get_row(self, r: usize) -> Self::Row {
            Vector2d::new([self[r][0], self[r][1]])
        }
        fn transpose(self) -> Self::Transpose {
            Matrix3x2::new([
                Vector2d::new([self[0][0], self[1][0]]),
                Vector2d::new([self[0][1], self[1][1]]),
                Vector2d::new([self[0][2], self[1][2]]),
            ])
        }
    }

    impl<T: Field> Matrix for Matrix2x4<T> {
        type Row = Vector2d<T>;
        type Column = Vector4d<T>;
        type Transpose = Matrix4x2<T>;

        fn get_row(self, r: usize) -> Self::Row {
            Vector2d::new([self[r][0], self[r][1]])
        }
        fn transpose(self) -> Self::Transpose {
            Matrix4x2::new([
                Vector2d::new([self[0][0], self[1][0]]),
                Vector2d::new([self[0][1], self[1][1]]),
                Vector2d::new([self[0][2], self[1][2]]),
                Vector2d::new([self[0][3], self[1][3]]),
            ])
        }
    }

    impl<T: Field> Matrix for Matrix3x2<T> {
        type Row = Vector3d<T>;
        type Column = Vector2d<T>;
        type Transpose = Matrix2x3<T>;

        fn get_row(self, r: usize) -> Self::Row {
            Vector3d::new([self[r][0], self[r][1], self[r][2]])
        }
        fn transpose(self) -> Self::Transpose {
            Matrix2x3::new([
                Vector3d::new([self[0][0], self[1][0], self[2][0]]),
                Vector3d::new([self[0][1], self[1][1], self[2][1]]),
            ])
        }
    }

    impl<T: Field> Matrix for Matrix3x3<T> {
        type Row = Vector3d<T>;
        type Column = Vector3d<T>;
        type Transpose = Matrix3x3<T>;

        fn get_row(self, r: usize) -> Self::Row {
            Vector3d::new([self[r][0], self[r][1], self[r][2]])
        }
        fn transpose(self) -> Self::Transpose {
            Matrix3x3::new([
                Vector3d::new([self[0][0], self[1][0], self[2][0]]),
                Vector3d::new([self[0][1], self[1][1], self[2][1]]),
                Vector3d::new([self[0][2], self[1][2], self[2][2]]),
            ])
        }
    }

    impl<T: Field> Matrix for Matrix3x4<T> {
        type Row = Vector3d<T>;
        type Column = Vector4d<T>;
        type Transpose = Matrix4x3<T>;

        fn get_row(self, r: usize) -> Self::Row {
            Vector3d::new([self[r][0], self[r][1], self[r][2]])
        }
        fn transpose(self) -> Self::Transpose {
            Matrix4x3::new([
                Vector3d::new([self[0][0], self[1][0], self[2][0]]),
                Vector3d::new([self[0][1], self[1][1], self[2][1]]),
                Vector3d::new([self[0][2], self[1][2], self[2][2]]),
                Vector3d::new([self[0][3], self[1][3], self[2][3]]),
            ])
        }
    }

    impl<T: Field> Matrix for Matrix4x2<T> {
        type Row = Vector4d<T>;
        type Column = Vector2d<T>;
        type Transpose = Matrix2x4<T>;

        fn get_row(self, r: usize) -> Self::Row {
            Vector4d::new([self[r][0], self[r][1], self[r][2], self[r][3]])
        }
        fn transpose(self) -> Self::Transpose {
            Matrix2x4::new([
                Vector4d::new([self[0][0], self[1][0], self[2][0], self[3][0]]),
                Vector4d::new([self[0][1], self[1][1], self[2][1], self[3][1]]),
            ])
        }
    }

    impl<T: Field> Matrix for Matrix4x3<T> {
        type Row = Vector4d<T>;
        type Column = Vector3d<T>;
        type Transpose = Matrix3x4<T>;

        fn get_row(self, r: usize) -> Self::Row {
            Vector4d::new([self[r][0], self[r][1], self[r][2], self[r][3]])
        }
        fn transpose(self) -> Self::Transpose {
            Matrix3x4::new([
                Vector4d::new([self[0][0], self[1][0], self[2][0], self[3][0]]),
                Vector4d::new([self[0][1], self[1][1], self[2][1], self[3][1]]),
                Vector4d::new([self[0][2], self[1][2], self[2][2], self[3][2]]),
            ])
        }
    }

    impl<T: Field> Matrix for Matrix4x4<T> {
        type Row = Vector4d<T>;
        type Column = Vector4d<T>;
        type Transpose = Matrix4x4<T>;

        fn get_row(self, r: usize) -> Self::Row {
            Vector4d::new([self[r][0], self[r][1], self[r][2], self[r][3]])
        }
        fn transpose(self) -> Self::Transpose {
            Matrix4x4::new([
                Vector4d::new([self[0][0], self[1][0], self[2][0], self[3][0]]),
                Vector4d::new([self[0][1], self[1][1], self[2][1], self[3][1]]),
                Vector4d::new([self[0][2], self[1][2], self[2][2], self[3][2]]),
                Vector4d::new([self[0][3], self[1][3], self[2][3], self[3][3]]),
            ])
        }
    }

    #[inline]
    #[allow(clippy::many_single_char_names)]
    unsafe fn det_sub_proc_unsafe<T: Field>(
        m: &Matrix4x4<T>,
        x: usize,
        y: usize,
        z: usize,
    ) -> Vector4d<T> {
        let s: &[T; 16] = m.as_ref();
        let a = Vector4d::new([
            *s.get_unchecked(4 + x),
            *s.get_unchecked(12 + x),
            *s.get_unchecked(x),
            *s.get_unchecked(8 + x),
        ]);
        let b = Vector4d::new([
            *s.get_unchecked(8 + y),
            *s.get_unchecked(8 + y),
            *s.get_unchecked(4 + y),
            *s.get_unchecked(4 + y),
        ]);
        let c = Vector4d::new([
            *s.get_unchecked(12 + z),
            *s.get_unchecked(z),
            *s.get_unchecked(12 + z),
            *s.get_unchecked(z),
        ]);

        let d = Vector4d::new([
            *s.get_unchecked(8 + x),
            *s.get_unchecked(8 + x),
            *s.get_unchecked(4 + x),
            *s.get_unchecked(4 + x),
        ]);
        let e = Vector4d::new([
            *s.get_unchecked(12 + y),
            *s.get_unchecked(y),
            *s.get_unchecked(12 + y),
            *s.get_unchecked(y),
        ]);
        let f = Vector4d::new([
            *s.get_unchecked(4 + z),
            *s.get_unchecked(12 + z),
            *s.get_unchecked(z),
            *s.get_unchecked(8 + z),
        ]);

        let g = Vector4d::new([
            *s.get_unchecked(12 + x),
            *s.get_unchecked(x),
            *s.get_unchecked(12 + x),
            *s.get_unchecked(x),
        ]);
        let h = Vector4d::new([
            *s.get_unchecked(4 + y),
            *s.get_unchecked(12 + y),
            *s.get_unchecked(y),
            *s.get_unchecked(8 + y),
        ]);
        let i = Vector4d::new([
            *s.get_unchecked(8 + z),
            *s.get_unchecked(8 + z),
            *s.get_unchecked(4 + z),
            *s.get_unchecked(4 + z),
        ]);
        let mut tmp = a.mul_elementwise(b.mul_elementwise(c));
        tmp += d.mul_elementwise(e.mul_elementwise(f));
        tmp += g.mul_elementwise(h.mul_elementwise(i));
        tmp -= a.mul_elementwise(e.mul_elementwise(i));
        tmp -= d.mul_elementwise(h.mul_elementwise(c));
        tmp -= g.mul_elementwise(b.mul_elementwise(f));
        tmp
    }
}

#[macro_export]
macro_rules! matrix {
    ($x: expr, $y: expr) => {
        Vector2d::new([$x, $y])
    };
    ($x: expr, $y: expr, $z: expr) => {
        Vector3d::new([$x, $y, $z])
    };
    ($w: expr, $x: expr, $y: expr, $z: expr) => {
        Vector4d::new([$w, $x, $y, $z])
    };
    ($($x: expr, $y: expr)=>*) => {
        [$(Vector2d::new([$x, $y])),*]
    };
    ($($x: expr, $y: expr, $z: expr)=>*) => {
        [$(Vector3d::new([$x, $y, $z])),*]
    };
    ($($w: expr, $x: expr, $y: expr, $z: expr)=>*) => {
        [$(Vector4d::new([$w, $x, $y, $z])),*]
    };
    ($($($x: expr),*);*) => {
        matrix!($($($x),*)=>*).into()
    };
}
