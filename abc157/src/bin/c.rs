#![allow(unused_imports)]
use proconio::{input, fastout, marker::*};
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize, m: usize);
    let mut num = vec![-1; n];
    for _ in 0..m {
        input!(s: Usize1, c: i32);
        // 2桁以上の整数で一番左が0となるクエリはバツ
        if n != 1 && s == 0 && c == 0 {
            println!("-1");
            return
        }
        // 既に埋まっていて数字が異なればバツ
        if num[s] != -1 && num[s] != c {
            println!("-1");
            return    
        }
        num[s] = c;
    }
    match n {
        1 => println!("{}", if num[0] == -1 { 0 } else { num[0] }),
        2 => {
            print!("{}",   if num[0] == -1 { 1 } else { num[0] });
            println!("{}", if num[1] == -1 { 0 } else { num[1] });
        },
        3 => {
            print!("{}",   if num[0] == -1 { 1 } else { num[0] });
            print!("{}",   if num[1] == -1 { 0 } else { num[1] });
            println!("{}", if num[2] == -1 { 0 } else { num[2] });
        },
        _ => unreachable!(),
    }
}
