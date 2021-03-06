use proconio::input;
 
fn main() {
    input!(x: i8);
    println!("{}", match x {
        x if x > 0 => x,
        _          => 0,
    });
}
