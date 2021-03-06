#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;

fn main() {
    input!(c: char);
    println!("{}", (c as u8 + 1) as char);
}
