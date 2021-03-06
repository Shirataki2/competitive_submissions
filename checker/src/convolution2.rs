#![allow(unused_macros)]
macro_rules! input { ($io:expr => $($name:ident: $t:ty),+) => { $(let $name: $t = $io.read();)* }; }
macro_rules! outln { ($io: expr) => { $io.write("\n".to_string()); }; ($io: expr => $fmt: expr) => {$io.write(format!(concat!($fmt, "\n")))}; ($io: expr => $fmt: expr, $($arg: tt)*) => { $io.write(format!(concat!($fmt, "\n"), $($arg)*)); }; }
macro_rules! out { ($io: expr => $fmt: expr) => {$io.write(format!($fmt, "\n"))}; ($io: expr => $fmt: expr, $($arg: tt)*) => { $io.write(format!($fmt, $($arg)*)); }; }

use io::IO;
use ntt::*;

fn main() {
    let mut io = IO::new(std::io::stdin(), std::io::stdout());
    input!(io => n: usize, m: usize);
    let mut a: Vec<i64> = io.vec(n);
    let mut b: Vec<i64> = io.vec(m);
    let x = multiply_for_any_mod(&mut a, &mut b, 1_000_000_007);
    let ans = x.iter().map(|&v| v.to_string()).collect::<Vec<_>>().join(" ");
    outln!(io => "{}", ans);
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

pub mod ntt {
    use std::marker::PhantomData;
    use std::ops::*;

    type Num = i64;

    pub trait ModuloPrimitive {
        fn modulo() -> Num;
        fn primitive_root() -> Num;
    }

    macro_rules! define_modulo_primitive {
        ($name:ident, $mod:expr, $proot:expr) => {
            #[derive(Debug)]
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
                let t = a / b;
                a -= t * b;
                std::mem::swap(&mut a, &mut b);
                u -= t * v;
                std::mem::swap(&mut u, &mut v);
            }
            ModInt::new::<Num>(u)
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
            let p = ModInt::<M>::new(M::primitive_root());
            for i in (0..).map(|i| 1 << i).take_while(|&i| i < n) {
                let mut w = p.pow((M::modulo() - 1) / (2 * i as Num));
                if inverse { w = 1 / w; }
                for k in 0..i {
                    let wn = w.pow(k as Num);
                    for j in (0..).map(|j| 2 * i * j).take_while(|&j| j < n) {
                        let s = f[j + k];
                        let t = f[j + k + i] * wn;
                        f[j + k] = s + t;
                        f[j + k + i] = s - t;
                    }
                }
            }
            if inverse {
                for i in 0..n { f[i] = f[i] / ModInt::<M>::new(n as Num) }
            }
        }

        pub fn multiply(f: &Vec<Num>, g: &Vec<Num>) -> Vec<Num> {
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
        let x = NTT1::multiply(&f, &g);
        let y = NTT2::multiply(&f, &g);
        let z = NTT3::multiply(&f, &g);
        let (m1, m2) = (Mod167772161::modulo(), Mod469762049::modulo());
        let m1_inv_m2 = 1 / M2::new(m1);
        let m12_inv_m3 = 1 / (M3::new(m1) * M3::new(m2));
        let mut ret = vec![0; x.len()];
        for i in 0..x.len() {
            let v1 = ((M2::new(y[i]) - M2::new(x[i])) * m1_inv_m2).value();
            let v2 = ((M3::new(z[i]) - (M3::new(x[i]) + M3::new(m1) * M3::new(v1))) * m12_inv_m3).value();
            ret[i] = add(x[i], add(mul(m1, v1, modulo), mul(mul(m1, m2, modulo), v2, modulo), modulo), modulo);
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