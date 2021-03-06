use proconio::input;
 
fn main() {
    input!(n: f64);
    let ni = n as u64;
    let nmax = n.sqrt() as u64;
    let mut ans = Vec::new();
    for i in 1..nmax+1 {
        if ni % i == 0 {
            ans.push(i);
            ans.push(ni/i);
        }
    }
    ans.sort();
    let mut pre = 0;
    for c in ans {
        if c != pre {
            println!("{}", c);
        }
        pre = c;
    }
}
