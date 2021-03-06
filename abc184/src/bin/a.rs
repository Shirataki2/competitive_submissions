use proconio::input;
 
fn main() {
    input!(a: i32, b: i32);
    input!(c: i32, d: i32);
    println!("{}", a * d - c * b);
}
