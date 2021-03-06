#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;

use procon_fft::FastFourierTransform;

type Fft = FastFourierTransform<f64>;

fn main() {
    input!(n: usize);
    let mut f = vec![0.0];
    let mut g = vec![0.0];
    for _ in 0..n {
        input!(x: f64, y: f64);
        f.push(x);
        g.push(y);
    }
    let f: Fft = f.into();
    let g: Fft = g.into();
    let x = &(f * g)[1..=2*n];
    let x = x.iter().map(|x| x.round() as i64).collect::<Vec<_>>();
    x.iter().for_each(|x| println!("{}", x));
}
