use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input!(
        h: i64, w: i64,
        ch: usize, cw: usize,
        dh: usize, dw: usize,
        s: [Chars; h]
    );
    let mut cost = vec![vec![1u64<<60; w as usize]; h as usize];
    let mut dq = VecDeque::<(i64, i64)>::new();
    dq.push_front(((ch-1) as i64, (cw-1) as i64));
    cost[ch-1][cw-1] = 0;
    loop {
        match dq.pop_front() {
            Some((y, x)) => {
                for j in -2..=2 {
                    for i in -2..=2 {
                        if (i, j) == (0, 0) { continue; }
                        if y + j < 0 || x + i < 0 { continue; }
                        if y + j >= h || x + i >= w { continue; }
                        
                        let ny: usize = (y + j) as usize;
                        let nx: usize = (x + i) as usize;
                        if s[ny][nx] == '#' { continue; }
                        let c = cost[y as usize][x as usize];
                        let dist = j.abs() + i.abs();
                        match dist {
                            1 => {
                                if cost[ny][nx] > c {
                                    cost[ny][nx] = c;
                                    dq.push_front((ny as i64, nx as i64));
                                }
                            },
                            _ => {
                                if cost[ny][nx] > c + 1 {
                                    cost[ny][nx] = c + 1;
                                    dq.push_back((ny as i64, nx as i64));
                                }
                            }
                        }
                    }
                }
            },
            None => break
        }
    }
    let ans = cost[dh-1][dw-1];
    if ans == 1u64<<60 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
