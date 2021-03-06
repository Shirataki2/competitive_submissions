fn main() {
    println!("Hello, world!");
}

pub mod modint {
    use std::ops::*;
    use std::marker::PhantomData;
    use std::mem::swap;

    pub type Num = i64;

    pub trait Modulo {
        fn modulo() -> Num;
    }

    // Macro
    // pub struct Mod7;
    // impl Modulo for Mod7 {
    //     fn modulo() -> Num { 7 }
    // }

    #[derive(Clone, Copy, PartialEq, PartialOrd)]
    pub struct ModInt<M>(Num, PhantomData<M>);

    impl<M> std::fmt::Debug for ModInt<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("ModInt")
             .field(&self.0)
             .finish()
        }
    }

    impl<M: Modulo> ModInt<M> {
        pub fn new<T: Into<Num>>(v: T) -> Self {
            let mut v = v.into();
            let m = M::modulo();
            if v >= m { v %= m; }
            if v < 0 { v = (v % m + m) % m; }
            ModInt(v, PhantomData)
        }

        fn internal_pow(&self, mut e: Num) -> Self {
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
            Self::new(result)
        }

        pub fn pow<T: Into<Num>>(&self, e: T) -> Self {
            self.internal_pow(e.into())
        }

        pub fn value(&self) -> Num { self.0 }

        pub fn inv<T: Into<Num>>(&self) -> Self {
            let (mut a, mut b, mut u, mut v) = (self.0, M::modulo(), 1, 0);
            while b > 0 {
                let t = a / b;
                a -= t * b;
                swap(&mut a, &mut b);
                u -= t * v;
                swap(&mut u, &mut v);
            }
            Self::new(u)
        }
    }

    impl<T, M> AddAssign<T> for ModInt<M>
    where
        Num: From<T>,
        M: Modulo
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

    impl<T, M> Add<T> for ModInt<M>
    where
        Num: From<T>,
        M: Modulo,
    {
        type Output = ModInt<M>;
        fn add(self, rhs: T) -> Self::Output {
            let mut res = self;
            res += rhs;
            res
        }
    }

    impl<T, M> SubAssign<T> for ModInt<M>
    where
        Num: From<T>,
        M: Modulo
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

    impl<T, M> Sub<T> for ModInt<M>
    where
        Num: From<T>,
        M: Modulo,
    {
        type Output = ModInt<M>;
        fn sub(self, rhs: T) -> Self::Output {
            let mut res = self;
            res -= rhs;
            res
        }
    }

    impl<T, M> MulAssign<T> for ModInt<M>
    where
        Num: From<T>,
        M: Modulo,
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

    impl<T, M> Mul<T> for ModInt<M>
    where
        Num: From<T>,
        M: Modulo,
    {
        type Output = ModInt<M>;
        fn mul(self, rhs: T) -> Self::Output {
            let mut res = self;
            res *= rhs;
            res
        }
    }

    impl<T, M> DivAssign<T> for ModInt<M>
    where
        Num: From<T>,
        M: Modulo,
    {
        fn div_assign(&mut self, rhs: T) {
            let mut rhs = Num::from(rhs);
            let m = M::modulo();
            if rhs >= m {
                rhs %= m;
            }
            let inv = ModInt::<M>::new(rhs).internal_pow(m - 2);
            self.0 *= inv.value();
            self.0 %= m;
        }
    }

    impl<T, M> Div<T> for ModInt<M>
    where
        Num: From<T>,
        M: Modulo,
    {
        type Output = ModInt<M>;
        fn div(self, rhs: T) -> Self::Output {
            let mut res = self;
            res /= rhs;
            res
        }
    }
}