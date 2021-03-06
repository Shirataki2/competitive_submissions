use proconio::input;
use itertools::Itertools;

fn main() {
    input!(n: usize, k: usize, t: [[usize; n]; n]);
    let mut ans = 0;
    for ord in (0..n).permutations(n) {
        let mut cost = 0;
        let mut pre = 0;
        for i in ord {
            cost += t[pre][i];
            pre = i;
        }
        if pre == 0 && cost == k {
            ans += 1;
        }
    }
    println!("{}", ans);
}
