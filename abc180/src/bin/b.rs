use proconio::input;

fn main() {
    input!(n: usize, x: [f64; n]);
    println!("{}", x.iter().fold(0f64, |acc, x| acc + x.abs()));
    println!("{}", x.iter().fold(0f64, |acc, x| acc + x * x).sqrt());
    println!("{}", x.iter().fold(0f64, |acc, x| if acc > x.abs() { acc } else { x.abs() }));
}
