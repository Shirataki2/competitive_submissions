use proconio::{input, fastout};

#[fastout]
fn main() {
    input!(n: usize, mut l: [u64; n]);
    let mut ans = 0;
    l.sort();
    for i in 0..n {
        for j in i..n {
            for k in j..n {
                if l[i] == l[j] || l[j] == l[k] || l[k] == l[i] {
                    continue;
                }
                if l[i] + l[j] > l[k] {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
