use proconio::{input, fastout};

#[fastout]
fn main() {
    input!(x: i64, k: i64, d: i64);
    let x = x.abs();
    let (q, r) = (x / d, x % d);
    if q >= k {
        println!("{}", x - k * d);
        return
    }
    let rem = k - q;
    if rem % 2 == 0 {
        println!("{}", r);
    } else {
        println!("{}", d - r);
    }
}
