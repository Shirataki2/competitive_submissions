#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

pub mod modint {
    use std::cell::RefCell;
    use std::ops::*;
    type Num = u64;
    thread_local!(
        static MOD: RefCell<Num> = RefCell::new(0);
    );
    pub fn set_modint<T>(v: T)
    where
        Num: From<T>,
    {
        MOD.with(|x| x.replace(Num::from(v)));
    }
    fn modulo() -> Num {
        MOD.with(|x| *x.borrow())
    }
    pub struct ModInt(Num);
    impl Clone for ModInt {
        fn clone(&self) -> ModInt {
            ModInt(self.0)
        }
    }
    impl Copy for ModInt {}
    impl ModInt {
        pub fn new<T>(v: T) -> ModInt
        where
            Num: From<T>,
        {
            let mut v = Num::from(v);
            let m = modulo();
            if v >= m {
                v %= m;
            }
            ModInt(v)
        }
        fn internal_pow(&self, mut e: Num) -> ModInt {
            let mut result = 1;
            let mut cur = self.0;
            let m = modulo();
            while e > 0 {
                if e & 1 == 1 {
                    result *= cur;
                    result %= m;
                }
                e >>= 1;
                cur = (cur * cur) % m;
            }
            ModInt(result)
        }
        pub fn pow<T>(&self, e: T) -> ModInt
        where
            Num: From<T>,
        {
            self.internal_pow(Num::from(e))
        }
        pub fn value(&self) -> Num {
            self.0
        }
    }
    impl From<ModInt> for Num {
        fn from(m: ModInt) -> Num {
            m.value()
        }
    }
    impl<T> AddAssign<T> for ModInt
    where
        Num: From<T>,
    {
        fn add_assign(&mut self, rhs: T) {
            let mut rhs = Num::from(rhs);
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }
            self.0 += rhs;
            if self.0 >= m {
                self.0 -= m;
            }
        }
    }
    impl<T> Add<T> for ModInt
    where
        Num: From<T>,
    {
        type Output = ModInt;
        fn add(self, rhs: T) -> Self::Output {
            let mut res = self;
            res += rhs;
            res
        }
    }
    impl<T> SubAssign<T> for ModInt
    where
        Num: From<T>,
    {
        fn sub_assign(&mut self, rhs: T) {
            let mut rhs = Num::from(rhs);
            let m = modulo();
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
    impl<T> Sub<T> for ModInt
    where
        Num: From<T>,
    {
        type Output = ModInt;
        fn sub(self, rhs: T) -> Self::Output {
            let mut res = self;
            res -= rhs;
            res
        }
    }
    impl<T> MulAssign<T> for ModInt
    where
        Num: From<T>,
    {
        fn mul_assign(&mut self, rhs: T) {
            let mut rhs = Num::from(rhs);
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }
            self.0 *= rhs;
            self.0 %= m;
        }
    }
    impl<T> Mul<T> for ModInt
    where
        Num: From<T>,
    {
        type Output = ModInt;
        fn mul(self, rhs: T) -> Self::Output {
            let mut res = self;
            res *= rhs;
            res
        }
    }
    impl<T> DivAssign<T> for ModInt
    where
        Num: From<T>,
    {
        fn div_assign(&mut self, rhs: T) {
            let mut rhs = Num::from(rhs);
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }
            let inv = ModInt(rhs).internal_pow(m - 2);
            self.0 *= inv.value();
            self.0 %= m;
        }
    }
    impl<T> Div<T> for ModInt
    where
        Num: From<T>,
    {
        type Output = ModInt;
        fn div(self, rhs: T) -> Self::Output {
            let mut res = self;
            res /= rhs;
            res
        }
    }
}

#[fastout]
fn main() {
    use modint::*;
    set_modint(998_244_353u64);
    input!(n: usize, s: usize, a: [usize; n]);
    let mut dp = vec![vec![ModInt::new(0u64); s + 1]; n + 1];
    dp[0][0] = ModInt::new(1u64);
    for i in 0..n {
        for j in 0..=s {
            dp[i+1][j] = dp[i+1][j] + dp[i][j] * ModInt::new(2u64);
            if j >= a[i] {
                dp[i+1][j] = dp[i+1][j] + dp[i][j - a[i]];
            }
        }
    }
    println!("{}", dp[n][s].value());
}
