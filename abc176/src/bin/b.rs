use proconio::input;
use proconio::marker::Chars;


fn main() {
    input!(n: Chars);
    let s = n.iter().fold(0, |acc, c| {
        acc + c.to_digit(10).unwrap_or(0)
    });
    println!("{}", if s % 9 == 0 { "Yes" } else { "No" });
}
