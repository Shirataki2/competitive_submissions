use proconio::input;
 
fn main() {
    input!(a: i64, b: i64);
    input!(c: i64, d: i64);
    let x = c - a;
    let y = d - b;
    let ans = match (x, y) {
        (0, 0)  => 0,
        (a, b)
            if a == b || a == -b
                => 1,
        (a, b)
            if a.abs() + b.abs() <= 3
                => 1,
        (a, b)
            if (a + b) % 2 == 0
                => 2,
        (a, b)
            if (a + b).abs() <= 3 || (a - b).abs() <= 3
                => 2,
        (a, b)
            if (a + b).abs() <= 6
                => 2,
        (_, _)  => 3
    };
    println!("{}", ans);
}
