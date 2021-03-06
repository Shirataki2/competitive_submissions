use proconio::input;
 
fn main() {
    input!(n: usize, w: i64);
    input!(a: [[i64; 3]; n]);
    let mut v = Vec::<(i64, i64)>::new();
    for p in a {
        v.push((p[0],  p[2]));
        v.push((p[1], -p[2]));
    }
    v.sort();
    let mut acc = 0;
    for (_, c) in v {
        acc += c;
        if acc > w {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
