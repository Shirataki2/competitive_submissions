use proconio::input;
 
fn main() {
    const MAX_D: usize = 50;
    input!(n: usize, x: usize, m: usize);
    let mut v = vec![vec![0usize; m]; MAX_D+1];
    let mut s = vec![vec![0u128; m]; MAX_D+1];
    for i in 0..m {
        v[0][i] = (i * i) % m;
        s[0][i] = i as u128;
    }
    for d in 0..MAX_D {
        for i in 0..m {
            v[d+1][i] = v[d][v[d][i]];
            s[d+1][i] = s[d][i] + s[d][v[d][i]];
        }
    }
    let mut ans: u128 = 0;
    let mut cur = x;
    for d in (0..=MAX_D).rev() {
        if (n & 1<<d) > 0 {
            ans += s[d][cur] as u128;
            cur = v[d][cur];
        }
    }
    println!("{}", ans);
}
