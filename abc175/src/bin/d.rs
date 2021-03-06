use proconio::{input, fastout};
use std::cmp::*;
use std::i128;

const MAX_BIT: usize = 60;

#[fastout]
fn main() {
    input!(n: usize, k: usize, p: [usize; n], c: [i128; n]);
    let p = p.iter().map(|&x| x - 1).collect::<Vec<usize>>();
    
    let mut next = vec![vec![0usize; n]; MAX_BIT];
    let mut val = vec![vec![0i128; n]; MAX_BIT];
    let mut all = vec![vec![0i128; n]; MAX_BIT];
    
    for v in 0..n {
        next[0][v] = p[v];
        val[0][v] = c[v];
        all[0][v] = c[v];
    }
    for d in 0..MAX_BIT-1 {
        for v in 0..n {
            next[d+1][v] = next[d][next[d][v]];
            val[d+1][v] = val[d][v] + val[d][next[d][v]];
            all[d+1][v] = max(all[d][v], val[d][v] + all[d][next[d][v]]);
        }
    }
    let mut ans: i128 = std::i128::MIN;
    for i in 0..n {
        let mut score = 0;
        let mut v = i;
        for d in (0..=MAX_BIT-1).rev() {
            if (k & 1usize << d) > 0 {
                ans = max(ans, score + all[d][v]);
                score += val[d][v];
                v = next[d][v];
            }
        }
    }
    println!("{}", ans);
}
