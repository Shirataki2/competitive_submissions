use proconio::input;

fn main() {
    input!(n: usize, mut a: [i64; n]);
    let mut ans = 0;
    for i in 0..n-1 {
        let dif = a[i+1] - a[i];
        if dif < 0 {
            ans -= dif;
            a[i+1] -= dif;
        }
    }
    println!("{}", ans);
}
