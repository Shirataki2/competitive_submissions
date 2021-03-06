use proconio::input;

type V = Vec<Vec<Vec<f64>>>;

fn search(x: usize, y: usize, z: usize, dp: &mut V) -> f64 {
    if dp[x][y][z] > 0f64 { return dp[x][y][z] };
    if x == 100 || y == 100 || z == 100 { return 0f64 };
    let s = (x + y + z) as f64;
    dp[x][y][z] =
    (x as f64) / s * (search(x + 1, y, z, dp) + 1f64) +
    (y as f64) / s * (search(x, y + 1, z, dp) + 1f64) +
    (z as f64) / s * (search(x, y, z + 1, dp) + 1f64);
    dp[x][y][z]
}
 
fn main() {
    input!(a: usize, b: usize, c: usize);
    let mut dp: V = vec![vec![vec![0f64; 101]; 101]; 101];
    println!("{}", search(a, b, c, &mut dp));
}
