use proconio::{input, fastout};

fn c(l: f64, k: u32, a: &[f64]) -> bool {
    let mut t = 0;
    for log in a.iter() {
        if l > *log { continue }
        t += ((*log + l - 1.0) / l).floor() as u32 - 1;
    }
    t <= k
}

#[fastout]
fn main() {
    input!(n: usize, k: u32, a: [f64; n]);
    let mut l = 1e-10;
    let mut r = 1e10;
    for _ in 0..50 {
        let m = (l + r) / 2.0;
        if c(m, k, &a) {
            r = m;
        } else {
            l = m;
        }
    }
    println!("{}", r.ceil());
}
