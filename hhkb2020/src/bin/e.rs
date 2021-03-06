#![allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
use std::cmp::*;
use modint::*;

#[fastout]
fn main() {
    set_modint(1_000_000_007i64);
    input!(h: usize, w: usize, s: [Chars; h]);
    let mut lights = vec![vec![0; w]; h];
    let mut k = 0;
    for y in 0..h {
        for x in 0..w {
            if s[y][x] != '.' { continue; }
            k += 1;
            let mut ctr = 1;
            // 右
            for dx in 1.. {
                if x + dx >= w || s[y][x + dx] != '.' { break; }
                ctr += 1;
            }
            // 左
            for dx in 1.. {
                if x < dx || s[y][x - dx] != '.' { break; }
                ctr += 1;
            }
            // 上
            for dy in 1.. {
                if y + dy >= h || s[y + dy][x] != '.' { break; }
                ctr += 1;
            }
            // 下
            for dy in 1.. {
                if y < dy || s[y - dy][x] != '.' { break; }
                ctr += 1;
            }
            lights[y][x] = ctr;
        }
    }
    let mut pow2 = vec![0; k as usize +1];
    let mut ans = ModInt::new(2).pow(k) * k;
    for y in 0..h {
        for x in 0..w {
            if lights[y][x] == 0 { continue; }
            if pow2[(k - lights[y][x]) as usize] == 0 {
                let p2 = k - lights[y][x];
                let v = ModInt::new(2).pow(p2 as i64);
                ans -= v;
                pow2[p2 as usize] = v.value();
            } else {
                ans -= pow2[(k - lights[y][x]) as usize];
            }
        }
    }
    println!("{}", ans.value());
}

pub mod modint {
    use std::cell::RefCell;
    use std::ops::*;
    use std::mem::swap;

    type Num = i64;
    thread_local!(
        static MOD: RefCell<Num> = RefCell::new(0);
    );

    pub fn set_modint<T>(v: T)
    where
        Num: From<T>
    {
        MOD.with(|x| x.replace(Num::from(v)));
    }

    pub fn modulo() -> Num {
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
            Num: From<T>
        {
            let mut v = Num::from(v);
            let m = modulo();
            if v >= m {
                v %= m;
            }
            if v < 0 {
                v = (v % m + m) % m;
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
            Num: From<T>
        {
            self.internal_pow(Num::from(e))
        }

        pub fn value(&self) -> Num {
            self.0
        }

        pub fn inv(&self) -> Self
        {
            let (mut a, mut b, mut u, mut v) = (self.0, modulo(), 1, 0);
            while b > 0 {
                let tmp = a / b;
                a -= tmp * b;
                swap(&mut a, &mut b);
                u -= tmp * v;
                swap(&mut u, &mut v);
            }
            ModInt::new::<i64>(u)
        }
    }

    impl From<ModInt> for Num {
        fn from(m: ModInt) -> Num {
            m.value()
        }
    }

    impl<T> AddAssign<T> for ModInt
    where
        Num: From<T>
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
        Num: From<T>
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
        Num: From<T>
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
        Num: From<T>
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
        Num: From<T>
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
        Num: From<T>
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
        Num: From<T>
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
        Num: From<T>
    {
        type Output = ModInt;
        fn div(self, rhs: T) -> Self::Output {
            let mut res = self;
            res /= rhs;
            res
        }
    }
}