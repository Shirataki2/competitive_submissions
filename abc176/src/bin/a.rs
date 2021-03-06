use proconio::input;
 
fn main() {
    input!(n: f64, x: f64, t: u32);
    println!("{}", (n / x).ceil() as u32 * t);
}
