use proconio::{input, fastout};

#[fastout]
fn main() {
    input!(mut x: [i32; 5]);
    println!("{}", 15 - x.iter().sum::<i32>());
}
