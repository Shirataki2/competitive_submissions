#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;

pub fn divisor(n: u64) -> Vec<u64> {
    let mut ret = Vec::new();
    for i in 1.. {
        if i * i > n { break }
        if n % i == 0 {
            ret.push(i);
            if i * i != n { ret.push(n / i); }
        }
    }
    ret.sort_unstable();
    ret
}

//#[fastout]
fn main() {
    //! kがnの約数でない場合
    //! n \equiv 1 (mod k)であればよいので
    //! n-1の1以外の約数であれば題意を満たす
    //! 
    //! kがnの約数である場合
    //! nは高々10^12なので約数を全部列挙し
    //! それぞれについて愚直に割っていき
    //! 割り切れなくなった値n'について
    //! n' % k == 1であれば結果に加える
    input!(n: u64);
    let div1 = divisor(n-1);
    let div2 = divisor(n);
    let mut ans = div1.len() - 1;
    for d in div2.iter() {
        if *d == 1 { continue; }
        let mut x = n;
        while x % d == 0 { x /= d; }
        if x % d == 1 { ans += 1; }
    }
    println!("{}", ans);
}
