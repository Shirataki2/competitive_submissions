use proconio::input;

fn main() {
    input!(sx: f64, sy: f64, gx: f64, gy: f64);
    let ans: f64 = (sx * gy + sy * gx) / (sy + gy);
    println!("{}", ans);
}
