use proconio::input;

fn main() {
    input!(s: String);
    let t = if s.ends_with('s') { "es" } else { "s" };
    println!("{}", s + t);
}
