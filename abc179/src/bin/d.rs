use proconio::input;
#[allow(unused_imports)]
use std::cmp::*;

type Mod = ntt::ModInt::<ntt::Mod998244353>;

fn main() {
    input!(n: usize, k: usize);
    let mut v = vec![Mod::new(0); n+1];
    for _ in 0..k {
        input!(l: usize, r: usize);
        for i in l..=r { v[i] += 1; }
    }
    let f = fps::FPS::new(v);
    let ans = (-f + Mod::new(1)).inv();
    if ans.0.len() >= n {
        println!("{}", ans.values()[n-1]);
    } else {
        println!("{}", 0);
    }
}

pub mod fps {
    use super::ntt::*;
    use super::num_trait::*;
    use std::ops::*;
    use std::cmp::*;

    impl<M: ModuloPrimitive> Zero for ModInt<M> {
        fn zero() -> Self { Self::new(0) }
        fn is_zero(&self) -> bool { self.value() == 0 }
    }

    #[derive(Debug, Clone)]
    pub struct FPS<M: ModuloPrimitive>(pub Vec<ModInt<M>>);

    impl <M: ModuloPrimitive> FPS<M> {
        pub fn new(v: Vec<ModInt<M>>) -> Self {
            Self(v)
        }

        pub fn with_size(size: usize) -> Self {
            let v = vec![ModInt::zero(); size];
            Self(v)
        }

        pub fn values(&self) -> Vec<i64> {
            self.0.iter().map(|&v| v.value()).collect()
        }

        #[inline]
        fn head(&self, n: usize) -> Self {
            Self::new(self.0.clone().drain(..min(n, self.0.len())).collect())
        }

        #[inline]
        fn rev(&self) -> Self {
            let mut res = self.0.clone();
            res.reverse();
            Self(res)
        }

        #[inline]
        fn cut(&mut self) {
            while !self.0.is_empty() && self.0.iter().next_back().unwrap().value() == 0 {
                self.0.pop();
            }
        }

        fn inner_gcd(x: &Self, y: &Self) -> Self {
            let (x, y) = (x.clone(), y.clone());
            if y.0.is_empty() { return x }
            let r = x % y.clone();
            Self::inner_gcd(&y, &r)
        }

        pub fn gcd(&self, r: &Self) -> Self {
            Self::inner_gcd(&self, &r)
        }

        pub fn diff(&self) -> Self {
            let n = self.0.len();
            let mut f = Self::with_size(n - 1);
            for i in 1..n {
                f[i-1] = self[i] * i as i64;
            }
            f
        }

        pub fn integral(&self) -> Self {
            let n = self.0.len();
            let mut f = Self::with_size(n + 1);
            for i in 0..n {
                f[i+1] = self[i] / (i + 1) as i64;
            }
            f
        }

        pub fn inv_degree(&self, mut deg: i64) -> Self {
            assert!(self[0] != ModInt::<M>::zero());
            if deg < 0 { deg = self.0.len() as i64; }
            let v = vec![ModInt::<M>::new(1) / self[0]];
            let mut res = Self::new(v);
            (0..).map(|i| 1 << i).take_while(|&i| i < deg as usize).for_each(|i| {
                let rres = res.clone() + res.clone();
                let mres = res.clone() * res.clone();
                res = ( rres - mres * self.head(i << 1)).head(i << 1);
            });
            res.0.resize(deg as usize, ModInt::zero());
            res
        }

        pub fn inv(&self) -> Self {
            self.inv_degree(self.0.len() as i64)
        }

        pub fn log_degree(&self, deg: i64) -> Self {
            assert!(self[0] == ModInt::<M>::new(1));
            let mut v = (self.diff() * self.inv_degree(deg)).integral();
            v.0.resize(deg as usize, ModInt::zero());
            v
        }

        pub fn log(&self) -> Self {
            self.log_degree(self.0.len() as i64)
        }

        pub fn exp_degree(&self, deg: i64) -> Self {
            assert!(self[0] == ModInt::<M>::zero());
            let one = ModInt::<M>::new(1);
            let mut v = Self::new(vec![one]);
            (0..).map(|i| 1 << i).take_while(|&i| i < deg as usize).for_each(|i| {
                v = v.clone() * (self.head(i << 1) - v.log_degree((i << 1) as i64) + one).head(i << 1);
            });
            v.0.resize(deg as usize, ModInt::zero());
            v
        }

        pub fn exp(&self) -> Self {
            self.exp_degree(self.0.len() as i64)
        }

        pub fn pow_degree(&self, n: usize, deg: i64) -> Self {
            let mut i = 0;
            while i < self.0.len() && self[i].value() == 0 { i += 1; }
            if i == self.0.len() { return Self::with_size(deg as usize); }
            if i * n >= deg as usize { return Self::with_size(deg as usize); }
            let k = self[i];
            let nm = ModInt::<M>::new(n as i64);
            let mut v = ((((self.clone() >> i) / k).log_degree(deg) * nm).exp_degree(deg) * k.pow(n as i64)) << (n * i);
            v.0.resize(deg as usize, ModInt::zero());
            v
        }

        pub fn pow(&self, n: usize) -> Self {
            self.pow_degree(n, self.0.len() as i64)
        }

        /// WIP
        pub fn sqrt_degree(&self, deg: i64) -> Self {
            let one = ModInt::<M>::new(1);
            let inv2 = one / 2;
            let mut v = Self::new(vec![one]);
            (0..).map(|i| 1 << i).take_while(|&i| i < deg as usize).for_each(|i| {
                v = (v.clone() + self.head(i << 1) * v.inv_degree((i << 1) as i64)).head(i << 1);
                v.0.iter_mut().for_each(|vi| *vi *= inv2);
            });
            v.0.resize(deg as usize, ModInt::zero());
            v

        }

        /// WIP
        pub fn sqrt(&self) -> Self {
            self.sqrt_degree(self.0.len() as i64)
        }
    }

    impl<M: ModuloPrimitive> Index<usize> for FPS<M> {
        type Output = ModInt<M>;
        #[inline]
        fn index(&self, idx: usize) -> &Self::Output {
            &self.0[idx]
        }
    }

    impl<M: ModuloPrimitive> IndexMut<usize> for FPS<M> {
        #[inline]
        fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
            &mut self.0[idx]
        }
    }

    impl<M: ModuloPrimitive> Neg for FPS<M> {
        type Output = Self;
        #[inline]
        fn neg(self) -> Self::Output {
            let mut v = self.0;
            v.iter_mut().for_each(|vi| *vi = -*vi);
            Self::new(v)
        }
    }

    impl<M: ModuloPrimitive> AddAssign<ModInt<M>> for FPS<M> {
        #[inline]
        fn add_assign(&mut self, rhs: ModInt<M>) {
            if self.0.is_empty() {
                self.0.resize(1, ModInt::zero());
            }
            self[0] += rhs;
        }
    }

    impl<M: ModuloPrimitive> AddAssign for FPS<M> {
        #[inline]
        fn add_assign(&mut self, rhs: FPS<M>) {
            if rhs.0.len() > self.0.len() { self.0.resize(rhs.0.len(), ModInt::zero()); }
            for i in 0..rhs.0.len() {
                self[i] += rhs[i];
            }
            self.cut();
        }
    }

    impl<M: ModuloPrimitive> Add<ModInt<M>> for FPS<M> {
        type Output = FPS<M>;
        #[inline]
        fn add(self, rhs: ModInt<M>) -> Self::Output {
            let mut x = self;
            x += rhs;
            x
        }
    }

    impl<M: ModuloPrimitive> Add for FPS<M> {
        type Output = FPS<M>;
        #[inline]
        fn add(self, rhs: Self) -> Self::Output {
            let mut x = self;
            x += rhs;
            x
        }
    }

    impl<M: ModuloPrimitive> SubAssign<ModInt<M>> for FPS<M> {
        #[inline]
        fn sub_assign(&mut self, rhs: ModInt<M>) {
            if self.0.is_empty() {
                self.0.resize(1, ModInt::zero());
            }
            self[0] -= rhs;
        }
    }

    impl<M: ModuloPrimitive> SubAssign for FPS<M> {
        #[inline]
        fn sub_assign(&mut self, rhs: FPS<M>) {
            if rhs.0.len() > self.0.len() { self.0.resize(rhs.0.len(), ModInt::zero()); }
            for i in 0..rhs.0.len() {
                self[i] -= rhs[i];
            }
            self.cut();
        }
    }

    impl<M: ModuloPrimitive> Sub<ModInt<M>> for FPS<M> {
        type Output = FPS<M>;
        #[inline]
        fn sub(self, rhs: ModInt<M>) -> Self::Output {
            let mut x = self;
            x -= rhs;
            x
        }
    }

    impl<M: ModuloPrimitive> Sub for FPS<M> {
        type Output = FPS<M>;
        #[inline]
        fn sub(self, rhs: Self) -> Self::Output {
            let mut x = self;
            x -= rhs;
            x
        }
    }

    impl<M: ModuloPrimitive> MulAssign<ModInt<M>> for FPS<M> {
        #[inline]
        fn mul_assign(&mut self, rhs: ModInt<M>) {
            for i in 0..self.0.len() {
                self[i] *= rhs;
            }
        }
    }

    impl<M: ModuloPrimitive> MulAssign for FPS<M> {
        #[inline]
        fn mul_assign(&mut self, rhs: FPS<M>) {
            let v = NumberTheoreticTransform::<M>::multiply_modint(&self.0, &rhs.0);
            self.0 = v;
        }
    }

    impl<M: ModuloPrimitive> Mul<ModInt<M>> for FPS<M> {
        type Output = FPS<M>;
        #[inline]
        fn mul(self, rhs: ModInt<M>) -> Self::Output {
            let mut x = self;
            x *= rhs;
            x
        }
    }

    impl<M: ModuloPrimitive> Mul for FPS<M> {
        type Output = FPS<M>;
        #[inline]
        fn mul(self, rhs: Self) -> Self::Output {
            let mut x = self;
            x *= rhs;
            x
        }
    }

    impl<M: ModuloPrimitive> DivAssign<ModInt<M>> for FPS<M> {
        #[inline]
        fn div_assign(&mut self, rhs: ModInt<M>) {
            let rinv = 1 / rhs;
            for i in 0..self.0.len() {
                self[i] *= rinv;
            }
        }
    }

    impl<M: ModuloPrimitive> DivAssign for FPS<M> {
        #[inline]
        fn div_assign(&mut self, rhs: FPS<M>) {
            assert!(!rhs.0.is_empty());
            assert!(rhs.0.iter().next_back().unwrap() != &ModInt::<M>::zero());
            self.cut();
            if self.0.len() < rhs.0.len() {
                self.0.clear();
                return
            }
            let need = self.0.len() - rhs.0.len() + 1;
            let v = (self.rev().head(need) * rhs.rev().inv_degree(need as i64)).head(need).rev();
            *self = v;
        }
    }

    impl<M: ModuloPrimitive> Div<ModInt<M>> for FPS<M> {
        type Output = FPS<M>;
        #[inline]
        fn div(self, rhs: ModInt<M>) -> Self::Output {
            let mut x = self;
            x /= rhs;
            x
        }
    }

    impl<M: ModuloPrimitive> Div for FPS<M> {
        type Output = FPS<M>;
        #[inline]
        fn div(self, rhs: Self) -> Self::Output {
            let mut x = self;
            x /= rhs;
            x
        }
    }

    impl<M: ModuloPrimitive> RemAssign for FPS<M> {
        fn rem_assign(&mut self, rhs: Self) {
            self.cut();
            let r = self.clone();
            let q = r / rhs.clone();
            *self -= q * rhs;
        }
    }

    impl<M: ModuloPrimitive> Rem for FPS<M> {
        type Output = FPS<M>;
        #[inline]
        fn rem(self, rhs: Self) -> Self::Output {
            let mut x = self;
            x %= rhs;
            x
        }
    }

    // <<=
    impl<M: ModuloPrimitive> ShlAssign<usize> for FPS<M> {
        #[inline]
        fn shl_assign(&mut self, x: usize) {
            let mut v = vec![ModInt::<M>::zero(); x];
            v.append(&mut self.0);
            self.0 = v;
        }
    }

    impl<M: ModuloPrimitive> Shl<usize> for FPS<M> {
        type Output = FPS<M>;
        #[inline]
        fn shl(self, rhs: usize) -> Self::Output {
            let mut x = self;
            x <<= rhs;
            x
        }
    }

    // >>=
    impl<M: ModuloPrimitive> ShrAssign<usize> for FPS<M> {
        #[inline]
        fn shr_assign(&mut self, x: usize) {
            self.0 = self.0.drain(x..).collect();
        }
    }

    impl<M: ModuloPrimitive> Shr<usize> for FPS<M> {
        type Output = FPS<M>;
        #[inline]
        fn shr(self, rhs: usize) -> Self::Output {
            let mut x = self;
            x >>= rhs;
            x
        }
    }
}

pub mod ntt {
    use std::marker::PhantomData;
    use std::ops::*;

    type Num = i64;

    pub trait ModuloPrimitive: Clone + Copy {
        fn modulo() -> Num;
        fn primitive_root() -> Num;
    }

    macro_rules! define_modulo_primitive {
        ($name:ident, $mod:expr, $proot:expr) => {
            #[derive(Debug, Clone, Copy)]
            pub struct $name;
            impl ModuloPrimitive for $name {
                fn modulo() -> i64 { $mod }
                fn primitive_root() -> i64 { $proot }
            }
        };
    }

    define_modulo_primitive!(Mod924844033, 924844033, 5);
    define_modulo_primitive!(Mod998244353, 998244353, 3);
    define_modulo_primitive!(Mod1012924417, 1012924417, 5);
    define_modulo_primitive!(Mod167772161, 167772161, 3);
    define_modulo_primitive!(Mod469762049, 469762049, 3);
    define_modulo_primitive!(Mod1224736769, 1224736769, 3);

    #[derive(Debug)]
    pub struct ModInt<M>(Num, PhantomData<M>);

    impl<M> Clone for ModInt<M> {
        fn clone(&self) -> ModInt<M> {
            ModInt(self.0, PhantomData)
        }
    }

    impl<M> Copy for ModInt<M> {}

    impl<M: ModuloPrimitive> ModInt<M> {
        pub fn new<T>(v: T) -> ModInt<M>
        where
            Num: From<T>
        {
            let mut v = Num::from(v);
            let m = M::modulo();
            if v >= m {
                v %= m;
            }
            if v < 0 {
                v = (v % m + m) % m;
            }
            ModInt(v, PhantomData)
        }

        fn internal_pow(&self, mut e: Num) -> ModInt<M> {
            let mut result = 1;
            let mut cur = self.0;
            let m = M::modulo();
            while e > 0 {
                if e & 1 == 1 {
                    result *= cur;
                    result %= m;
                }
                e >>= 1;
                cur = (cur * cur) % m;
            }
            ModInt(result, PhantomData)
        }

        pub fn pow<T>(&self, e: T) -> ModInt<M>
        where
            Num: From<T>
        {
            self.internal_pow(Num::from(e))
        }

        pub fn value(&self) -> Num {
            self.0
        }

        pub fn inv(&self) -> Self
        {
            let (mut a, mut b, mut u, mut v) = (self.0, M::modulo(), 1, 0);
            while b > 0 {
                let tmp = a / b;
                a -= tmp * b;
                std::mem::swap(&mut a, &mut b);
                u -= tmp * v;
                std::mem::swap(&mut u, &mut v);
            }
            ModInt::new::<Num>(u)
        }
    }

    impl<M> Neg for ModInt<M>
    where
        M: ModuloPrimitive
    {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Self::new(M::modulo() - self.0)
        }
    }

    impl<T, M> AddAssign<T> for ModInt<M>
    where
        Num: From<T>,
        M: ModuloPrimitive
    {
        fn add_assign(&mut self, rhs: T) {
            let mut rhs = Num::from(rhs);
            let m = M::modulo();
            if rhs >= m {
                rhs %= m;
            }
            self.0 += rhs;
            if self.0 >= m {
                self.0 -= m;
            }
        }
    }

    impl<M> AddAssign for ModInt<M>
    where
        M: ModuloPrimitive
    {
        fn add_assign(&mut self, rhs: ModInt<M>) {
            *self += rhs.value();
        }
    }

    impl<T, M> Add<T> for ModInt<M>
    where
        Num: From<T>,
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn add(self, rhs: T) -> Self::Output {
            let mut res = self;
            res += rhs;
            res
        }
    }

    impl<M> Add for ModInt<M>
    where
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn add(self, rhs: ModInt<M>) -> Self::Output {
            let mut res = self;
            res += rhs.value();
            res
        }
    }

    impl<M> Add<ModInt<M>> for Num
    where
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn add(self, rhs: ModInt<M>) -> Self::Output {
            let mut res = ModInt::<M>::new(self);
            res += rhs.value();
            res
        }
    }

    impl<T, M> SubAssign<T> for ModInt<M>
    where
        Num: From<T>,
        M: ModuloPrimitive
    {
        fn sub_assign(&mut self, rhs: T) {
            let mut rhs = Num::from(rhs);
            let m = M::modulo();
            if rhs >= m {
                rhs %= m;
            }
            if rhs > 0 {
                self.0 += m - rhs;
            }
            if self.0 >= m {
                self.0 -= m;
            }
        }
    }

    impl<M> SubAssign for ModInt<M>
    where
        M: ModuloPrimitive
    {
        fn sub_assign(&mut self, rhs: ModInt<M>) {
            *self -= rhs.value();
        }
    }

    impl<T, M> Sub<T> for ModInt<M>
    where
        Num: From<T>,
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn sub(self, rhs: T) -> Self::Output {
            let mut res = self;
            res -= rhs;
            res
        }
    }

    impl<M> Sub for ModInt<M>
    where
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn sub(self, rhs: ModInt<M>) -> Self::Output {
            let mut res = self;
            res -= rhs.value();
            res
        }
    }

    impl<M> Sub<ModInt<M>> for Num
    where
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn sub(self, rhs: ModInt<M>) -> Self::Output {
            let mut res = ModInt::<M>::new(self);
            res -= rhs.value();
            res
        }
    }

    impl<T, M> MulAssign<T> for ModInt<M>
    where
        Num: From<T>,
        M: ModuloPrimitive
    {
        fn mul_assign(&mut self, rhs: T) {
            let mut rhs = Num::from(rhs);
            let m = M::modulo();
            if rhs >= m {
                rhs %= m;
            }
            self.0 *= rhs;
            self.0 %= m;
        }
    }

    impl<M> MulAssign for ModInt<M>
    where
        M: ModuloPrimitive
    {
        fn mul_assign(&mut self, rhs: ModInt<M>) {
            *self *= rhs.value();
        }
    }

    impl<T, M> Mul<T> for ModInt<M>
    where
        Num: From<T>,
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn mul(self, rhs: T) -> Self::Output {
            let mut res = self;
            res *= rhs;
            res
        }
    }

    impl<M> Mul for ModInt<M>
    where
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn mul(self, rhs: ModInt<M>) -> Self::Output {
            let mut res = self;
            res *= rhs.value();
            res
        }
    }

    impl<M> Mul<ModInt<M>> for Num
    where
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn mul(self, rhs: ModInt<M>) -> Self::Output {
            let mut res = ModInt::<M>::new(self);
            res *= rhs.value();
            res
        }
    }

    impl<T, M> DivAssign<T> for ModInt<M>
    where
        Num: From<T>,
        M: ModuloPrimitive
    {
        fn div_assign(&mut self, rhs: T) {
            let mut rhs = Num::from(rhs);
            let m = M::modulo();
            if rhs >= m {
                rhs %= m;
            }
            let inv = ModInt::<M>(rhs, PhantomData).internal_pow(m - 2);
            self.0 *= inv.value();
            self.0 %= m;
        }
    }

    impl<M> DivAssign for ModInt<M>
    where
        M: ModuloPrimitive
    {
        fn div_assign(&mut self, rhs: ModInt<M>) {
            *self /= rhs.value();
        }
    }

    impl<T, M> Div<T> for ModInt<M>
    where
        Num: From<T>,
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn div(self, rhs: T) -> Self::Output {
            let mut res = self;
            res /= rhs;
            res
        }
    }

    impl<M> Div for ModInt<M>
    where
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn div(self, rhs: ModInt<M>) -> Self::Output {
            let mut res = self;
            res /= rhs.value();
            res
        }
    }

    impl<M> Div<ModInt<M>> for Num
    where
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn div(self, rhs: ModInt<M>) -> Self::Output {
            let mut res = ModInt::<M>::new(self);
            res /= rhs.value();
            res
        }
    }
    
    impl<M> PartialEq for ModInt<M>
    where
        M: ModuloPrimitive
    {
        fn eq(&self, rhs: &Self) -> bool {
            self.value() == rhs.value()
        }
    }

    pub struct NumberTheoreticTransform<M>(PhantomData<M>);

    impl<M> NumberTheoreticTransform<M>
    where
        M: ModuloPrimitive
    {
        fn bit_reverse(f: &mut Vec<ModInt<M>>) {
            let mut i = 0;
            for j in 1..f.len()-1 {
                let mut k = f.len() >> 1;
                while { i ^= k; k > i } { k >>= 1; }
                if i > j { f.swap(i, j); }
            }
        }

        fn dft(f: &mut Vec<ModInt<M>>, inverse: bool) {
            let n = f.len();
            NumberTheoreticTransform::<M>::bit_reverse(f);
            let proot = ModInt::<M>::new(M::primitive_root());
            for i in (0..).map(|i| 1 << i).take_while(|&i| i < n) {
                let mut w = proot.pow((M::modulo() - 1) / (2 * i as Num));
                if inverse { w = 1 / w; }
                for k in 0..i {
                    let wn = w.pow(k as Num);
                    for j in (0..).map(|j| 2 * i * j).take_while(|&j| j < n) {
                        let left = f[j + k];
                        let right = f[j + k + i] * wn;
                        f[j + k] = left + right;
                        f[j + k + i] = left - right;
                    }
                }
            }
            if inverse {
                f.iter_mut().for_each(|fi| { *fi /= ModInt::<M>::new(n as Num); })
            }
        }

        pub fn multiply(f: &[Num], g: &[Num]) -> Vec<Num> {
            let m = f.len() + g.len() - 1;
            let n = m.next_power_of_two();
            let zero = ModInt::<M>::new(0);
            let mut ff = vec![zero; n];
            let mut gg = vec![zero; n];
            for i in 0..f.len() { ff[i] += ModInt::<M>::new(f[i]); }
            for i in 0..g.len() { gg[i] += ModInt::<M>::new(g[i]); }
            NumberTheoreticTransform::<M>::dft(&mut ff, false);
            NumberTheoreticTransform::<M>::dft(&mut gg, false);
            for i in 0..n { ff[i] *= gg[i]; }
            NumberTheoreticTransform::<M>::dft(&mut ff, true);
            ff.resize(m, zero);
            ff.iter().map(|&v| v.value()).collect()
        }

        pub fn multiply_modint(f: &[ModInt<M>], g: &[ModInt<M>]) -> Vec<ModInt<M>> {
            let m = f.len() + g.len();
            let n = m.next_power_of_two();
            let zero = ModInt::<M>::new(0);
            let mut ff = vec![zero; n];
            let mut gg = vec![zero; n];
            for i in 0..f.len() { ff[i] += f[i]; }
            for i in 0..g.len() { gg[i] += g[i]; }
            NumberTheoreticTransform::<M>::dft(&mut ff, false);
            NumberTheoreticTransform::<M>::dft(&mut gg, false);
            for i in 0..n { ff[i] *= gg[i]; }
            NumberTheoreticTransform::<M>::dft(&mut ff, true);
            ff.resize(m-1, zero);
            ff
        }
    }

    pub fn multiply_for_any_mod(f: &mut Vec<Num>, g: &mut Vec<Num>, modulo: Num) -> Vec<Num> {
        f.iter_mut().for_each(|v| *v %= modulo);
        g.iter_mut().for_each(|v| *v %= modulo);
        let f = f.to_vec();
        let g = g.to_vec();
        type M2 = ModInt<Mod469762049>;
        type M3 = ModInt<Mod1224736769>;
        type NTT1 = NumberTheoreticTransform::<Mod167772161>;
        type NTT2 = NumberTheoreticTransform::<Mod469762049>;
        type NTT3 = NumberTheoreticTransform::<Mod1224736769>;
        let ntt1 = NTT1::multiply(&f, &g);
        let ntt2 = NTT2::multiply(&f, &g);
        let ntt3 = NTT3::multiply(&f, &g);
        let (m1, m2) = (Mod167772161::modulo(), Mod469762049::modulo());
        let m1_inv_m2 = 1 / M2::new(m1);
        let m12_inv_m3 = 1 / (M3::new(m1) * M3::new(m2));
        let mut ret = vec![0; ntt1.len()];
        for i in 0..ntt1.len() {
            let v1 = ((M2::new(ntt2[i]) - M2::new(ntt1[i])) * m1_inv_m2).value();
            let v2 = ((M3::new(ntt3[i]) - (M3::new(ntt1[i]) + M3::new(m1) * M3::new(v1))) * m12_inv_m3).value();
            ret[i] = add(ntt1[i], add(mul(m1, v1, modulo), mul(mul(m1, m2, modulo), v2, modulo), modulo), modulo);
        }
        ret
    }

    fn add(mut x: Num, y: Num, modulo: Num) -> Num {
        x += y;
        if x >= modulo {
            x -= modulo;
        }
        x
    }

    fn mul(x: Num, y: Num, modulo: Num) -> Num {
        x * y % modulo
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
                fn is_one(&self) -> bool { 1.0 - 1e-6  < *self && 1.0 + 1e-6  > *self }
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
        )*};
    }

    integer_primitives!(u128 u64 u32 u16 u8 usize i128 i64 i32 i16 i8 isize);
    signed_int_primitives!(i128 i64 i32 i16 i8 isize);
    unsigned_int_primitives!(u128 u64 u32 u16 u8 usize);
    floating_primitives!(f32 f64);
}
