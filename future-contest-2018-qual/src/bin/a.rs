#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;
use std::time::Instant;
use rand::prelude::*;

const N: usize = 100;
const POINT_N: usize = 1_000;
const TIME_LIMIT: u128 = 5_970_000_000;

struct Solver {
    a: Vec<Vec<i64>>,
    rng: ThreadRng
}

fn clamp(x: i64, a: i64, b: i64) -> i64 {
    min(max(x, a), b)
}

#[allow(dead_code)]
enum SimulateType {
    Random,
    Hill,
    Annealing,
}

impl Solver {
    pub fn new(a: Vec<Vec<i64>>) -> Solver {
        let rng = thread_rng();
        Solver { a, rng }
    }

    fn gen_point(&mut self) -> (i64, i64, i64) {
        let x = self.rng.gen_range(0, N as i64);
        let y = self.rng.gen_range(0, N as i64);
        let h = self.rng.gen_range(1, N as i64 + 1);
        (x, y, h)
    }

    fn near_point(&mut self, point: &(i64, i64, i64), diff: (i64, i64, i64)) -> (i64, i64, i64) {
        let x = clamp(point.0 + self.rng.gen_range(-diff.0, diff.0+1), 0, N as i64 - 1);
        let y = clamp(point.1 + self.rng.gen_range(-diff.1, diff.1+1), 0, N as i64 - 1);
        let h = clamp(point.2 + self.rng.gen_range(-diff.2, diff.2+1), 1, N as i64);
        (x, y, h)
    }

    pub fn gen_random(&mut self) -> Vec<(i64, i64, i64)> {
        let mut v = Vec::with_capacity(POINT_N);
        for _ in 0..POINT_N {
            v.push(self.gen_point());
        }
        v
    }

    pub fn simulate(&mut self, ty: SimulateType) -> Vec<(i64, i64, i64)> {
        match ty {
            SimulateType::Random => self.simulate_random(),
            SimulateType::Hill => self.simulate_hc(),
            SimulateType::Annealing => self.simulate_sa(),
        }
    }

    pub fn simulate_random(&mut self) -> Vec<(i64, i64, i64)> {
        let timer = Instant::now();
        let v = self.gen_random();
        let mut best_score = self.eval(&v);
        let mut best_output = v;
        let mut ctr = 0;
        while timer.elapsed().as_nanos() < TIME_LIMIT {
            let v = self.gen_random();
            let score = self.eval(&v);
            if score < best_score {
                best_score = score;
                best_output = v;
            }
            ctr += 1;
        }
        eprintln!("loop: {}", ctr);
        best_output
    }

    pub fn simulate_hc(&mut self) -> Vec<(i64, i64, i64)> {
        let timer = Instant::now();
        let v = self.gen_random();
        let mut diff = self.make_diff(&v);
        let mut best_score = diff.iter().fold(0, |acc, v| acc + v.iter().fold(0, |acc, x| acc + x.abs()));
        let mut best_output = v.clone();
        let mut ctr = 0;
        while timer.elapsed().as_nanos() < TIME_LIMIT {
            let i = self.rng.gen_range(0, POINT_N);
            let before = best_output[i];
            let after = self.near_point(&before, (3, 3, 5));
            self.update_diff(&mut diff, &before, &after);
            let score = diff.iter().fold(0, |acc, v| acc + v.iter().fold(0, |acc, x| acc + x.abs()));
            if score < best_score {
                best_score = score;
                best_output[i] = after;
            } else {
                self.update_diff(&mut diff, &after, &before);
            }
            ctr += 1;
        }
        eprintln!("Score    : {}", 200_000_000 - best_score);
        eprintln!("Expected : {}", 50 * (200_000_000 - best_score));
        eprintln!("Loop     : {}", ctr);
        best_output
    }

    pub fn simulate_sa(&mut self) -> Vec<(i64, i64, i64)> {
        let timer = Instant::now();
        let v = self.gen_random();
        let mut diff = self.make_diff(&v);
        let mut best_score = diff.iter().fold(0, |acc, v| acc + v.iter().fold(0, |acc, x| acc + x.abs()));
        let mut best_output = v.clone();
        let mut ctr = 0;
        while timer.elapsed().as_nanos() < TIME_LIMIT {
            let t = timer.elapsed().as_nanos() as f64 / TIME_LIMIT as f64;
            let mut i = self.rng.gen_range(0, POINT_N);
            while best_output[i].2 > (10.0 + 110.0 * (1.0 - t)) as i64 {
                i = self.rng.gen_range(0, POINT_N);
            }
            let before = best_output[i];
            let after = self.near_point(&before, (1, 1, 1));
            self.update_diff(&mut diff, &before, &after);
            let score = diff.iter().fold(0, |acc, v| acc + v.iter().fold(0, |acc, x| acc + x.abs()));
            let p = (1.0 - t).powf(4.0);
            if score < best_score || p > self.rng.gen_range(0.0, 1.0) {
                best_score = score;
                best_output[i] = after;
            } else {
                self.update_diff(&mut diff, &after, &before);
            }
            ctr += 1;
        }
        eprintln!("Score    : {}", 200_000_000 - best_score);
        eprintln!("Expected : {}", 50 * (200_000_000 - self.eval(&best_output)));
        eprintln!("Loop     : {}", ctr);
        best_output
    }

    pub fn eval(&mut self, v: &[(i64, i64, i64)]) -> i64 {
        let mut res = 0;
        let mut board = vec![vec![0; N]; N];
        for &(x, y, h) in v.iter() {
            board[y as usize][x as usize] += h;
            for l1 in 1..h {
                let d = h - l1;
                let mut xi = x;
                let mut yi = y - d;
                for (dx, dy) in &[(1, 1), (-1, 1), (-1, -1), (1, -1)] {
                    for _ in 0..d {
                        xi += dx; yi += dy;
                        if self.is_valid_coord(xi, yi) {
                            board[yi as usize][xi as usize] += l1;
                        }
                    }
                }
            }
        }
        for y in 0..N {
            for x in 0..N {
                res += (self.a[y][x] - board[y][x]).abs();
            }
        }
        res
    }

    /// 差分のみを更新する
    fn update_diff(&mut self, diff: &mut Vec<Vec<i64>>, before: &(i64, i64, i64), after: &(i64, i64, i64)) {
        let (x, y, h) = *before;
        diff[y as usize][x as usize] += h;
        for l1 in 1..h {
            let d = h - l1;
            let mut xi = x;
            let mut yi = y - d;
            for (dx, dy) in &[(1, 1), (-1, 1), (-1, -1), (1, -1)] {
                for _ in 0..d {
                    xi += dx; yi += dy;
                    if self.is_valid_coord(xi, yi) {
                        diff[yi as usize][xi as usize] += l1;
                    }
                }
            }
        }
        let (x, y, h) = *after;
        diff[y as usize][x as usize] -= h;
        for l1 in 1..h {
            let d = h - l1;
            let mut xi = x;
            let mut yi = y - d;
            for (dx, dy) in &[(1, 1), (-1, 1), (-1, -1), (1, -1)] {
                for _ in 0..d {
                    xi += dx; yi += dy;
                    if self.is_valid_coord(xi, yi) {
                        diff[yi as usize][xi as usize] -= l1;
                    }
                }
            }
        }
    }

    /// 正解盤面との差分を各マスについて計算
    fn make_diff(&mut self, v: &[(i64, i64, i64)]) -> Vec<Vec<i64>> {
        let mut res = vec![vec![0; N]; N];
        let mut board = vec![vec![0; N]; N];
        for &(x, y, h) in v.iter() {
            board[y as usize][x as usize] += h;
            for l1 in 1..h {
                let d = h - l1;
                let mut xi = x;
                let mut yi = y - d;
                for (dx, dy) in &[(1, 1), (-1, 1), (-1, -1), (1, -1)] {
                    for _ in 0..d {
                        xi += dx; yi += dy;
                        if self.is_valid_coord(xi, yi) {
                            board[yi as usize][xi as usize] += l1;
                        }
                    }
                }
            }
        }
        for y in 0..N {
            for x in 0..N {
                res[y][x] = self.a[y][x] - board[y][x];
            }
        }
        res
    }

    fn is_valid_coord(&self, x: i64, y: i64) -> bool {
        0 <= x && x < N as i64 && 0 <= y && y < N as i64
    }
}

#[fastout]
fn main() {
    input!(a: [[i64; N]; N]);
    let mut solver = Solver::new(a);
    let v = solver.simulate(SimulateType::Annealing);
    println!("{}", v.len());
    for (x, y, h) in v.iter() {
        println!("{} {} {}", x, y, h);
    }
}
