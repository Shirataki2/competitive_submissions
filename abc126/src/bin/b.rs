#![allow(unused_imports)]
use proconio::{input, fastout, marker::*};
use std::cmp::*;

#[fastout]
fn main() {
    input!(s: String);
    let (first, second) = s.split_at(2);
    let first: u64 = first.parse().unwrap();
    let second: u64 = second.parse().unwrap();
    let ans = match (first, second) {
        (yy, mm)
        if !is_month_range(yy) && is_month_range(mm)
            => { "YYMM" },
        (mm, yy)
        if !is_month_range(yy) && is_month_range(mm)
            => { "MMYY" },
        (mm, yy)
        if !is_month_range(yy) && !is_month_range(mm)
            => { "NA" },
        _ 
            => { "AMBIGUOUS" }
    };
    println!("{}", ans);
}

fn is_month_range(d: u64) -> bool {
    01 <= d && d <= 12
}