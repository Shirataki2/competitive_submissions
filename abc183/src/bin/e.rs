use proconio::input;
 
fn main() {
    const MOD: u64 = 1000000007;
    input!(h: usize, w: usize);
    input!(s: [String; h]);
    let m = (0..h).map(|i| {
        let cs = &s[i];
        cs.chars().map(|c| {
            match c {
                '#' => 1u8,
                 _  => 0u8,
            }
        }).collect::<Vec<u8>>()
    }).collect::<Vec<Vec<u8>>>();
    let mut dp = vec![vec![0u64; w]; h];
    let mut x = vec![vec![0u64; w]; h];
    let mut y = vec![vec![0u64; w]; h];
    let mut xy = vec![vec![0u64; w]; h];
    dp[0][0] = 1;
    for r in 0..h {
        for c in 0..w {
            if (r, c) == (0, 0) { continue; }
            if m[r][c] == 1 { continue; }
            if c > 0 {
                x[r][c] = (x[r][c - 1] + dp[r][c - 1]) % MOD;
            }
            if r > 0 {
                y[r][c] = (y[r - 1][c] + dp[r - 1][c]) % MOD;
            }
            if r > 0 && c > 0 {
                xy[r][c] = (xy[r - 1][c - 1] + dp[r - 1][c - 1]) % MOD;
            }
            dp[r][c] = (x[r][c] + y[r][c] + xy[r][c]) % MOD;
        }
    }
    println!("{}", dp[h-1][w-1] % MOD);
}
