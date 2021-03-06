use proconio::input;
 
fn main() {
    input!(n: usize);
    let mut ctr = 0;
    let mut ans = false;
    for _ in 0..n {
        input!(a: u8, b: u8);
        if a == b {
            ctr += 1;
        } else {
            ctr = 0;
        }
        if ctr >= 3 {
            ans = true;
            break;
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
