use proconio::input;
 
fn main() {
    input!(n: u64);
    let mut ans = 0;
    for i in 1..=n-1 {
        ans += ((n as f64 - 1.0) / (i as f64)).floor() as i32;
    }
    println!("{}", ans);
}
