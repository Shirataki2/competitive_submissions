use proconio::{input, fastout};

#[fastout]
fn main() {
    input!(s: String);
    let ans = match s.as_str() {
        "SSS" => 0,
        "RRR" => 3,
        "RRS" => 2,
        "SRR" => 2,
        _     => 1,
    };
    println!("{}", ans);
}
