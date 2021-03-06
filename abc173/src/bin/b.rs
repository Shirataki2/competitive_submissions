use proconio::{input, fastout};
#[allow(unused_imports)]
use std::cmp::*;

#[fastout]
fn main() {
    input!(n: usize);
    let (mut ac, mut wa, mut tle, mut re) = (0, 0, 0, 0);
    for _ in 0..n {
        input!(s: String);
        if s == "AC" { ac += 1}
        if s == "WA" { wa += 1}
        if s == "TLE" { tle += 1}
        if s == "RE" { re += 1}
    }
    println!("AC x {}", ac);
    println!("WA x {}", wa);
    println!("TLE x {}", tle);
    println!("RE x {}", re);
}
