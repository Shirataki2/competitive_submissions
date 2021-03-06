use proconio::input;
use std::cmp::{max, min};
 
fn dist(from: (i32, i32, i32), to: (i32, i32, i32)) -> i32 {
    (from.0 - to.0).abs() + (from.1 - to.1).abs() + max(0, to.2 - from.2)
}

fn main() {
    input!(n: usize, coords: [(i32, i32, i32); n]);
    let mut dp: Vec<Vec<i32>> = vec![vec![1<<30; n]; 1 << n];
    dp[1][0] = 0;
    for bit in 0..1<<n {
        for v in 0..n {
            for u in 0..n {
                if (bit & (1 << u)) > 0 { continue; }
                let nbit: usize = bit | (1 << u);
                dp[nbit][u] = min(dp[nbit][u], dp[bit][v] + dist(coords[v], coords[u]));
            }
        }
    }
    let mut ans = 1 << 30;
    for v in 0..n {
        ans = min(ans, dp[(1<<n)-1][v] + dist(coords[v], coords[0]));
    }
    println!("{}", ans);
}
