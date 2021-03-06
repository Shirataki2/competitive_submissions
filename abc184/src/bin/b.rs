use proconio::input;
use std::cmp::max;

fn main() {
    input!(_n: u64, a0: i64);
    input!(s: String);
    let ans: i64 = s.chars().fold(a0, |sum, c| {
        let c = match c {
            'o' => 1,
            _   => -1,
        };
        max(0, sum + c)
    });
    println!("{}", ans);
}
