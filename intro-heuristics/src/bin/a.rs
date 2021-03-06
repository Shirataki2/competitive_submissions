#![allow(unused_imports)]
use proconio::{input, fastout};
use std::cmp::*;
use std::convert::*;
use std::time::Instant;
use rand::{thread_rng, Rng, rngs::ThreadRng};

const D: usize = 365;
const TIME_LIMIT: u128 = 1_970_000_000;
const TYPES: usize = 26;

pub struct Solver {
    pub decay_factor: [i64; TYPES],
    pub sat: [[i64; TYPES]; D],
    last: [i64; TYPES],
    rng: ThreadRng,
}

impl Solver {
    pub fn new(decay_factor: [i64; TYPES], sat: [[i64; TYPES]; D]) -> Solver {
        Solver {
            decay_factor,
            sat,
            last: [0i64; TYPES],
            rng: thread_rng(),
        }
    }

    pub fn generate_random(&mut self) -> [usize; D] {
        let mut t = [0usize; D];
        for i in 0..D {
            let ty = self.rng.gen_range(0, TYPES as usize);
            t[i] = ty;
        }
        t
    }

    pub fn solve_random(&mut self) -> [usize; D] {
        let timer = Instant::now();
        let mut best = [0usize; D];
        let mut best_score = -1_000_000_000_000;
        let mut ctr = 0;
        while timer.elapsed().as_nanos() < TIME_LIMIT {
            let v = self.generate_random();
            let score = self.calc_scores(v);
            if score > best_score {
                best_score = score;
                best = v;
            }
            ctr += 1;
        }
        eprintln!("Loop : {}", ctr);
        eprintln!("Score: {}", best_score);
        best
    }

    pub fn solve_hill_climbing(&mut self) -> [usize; D] {
        let timer = Instant::now();
        let mut best = self.generate_random();
        let mut best_score = -1_000_000_000_000;
        let mut ctr = 0;
        while timer.elapsed().as_nanos() < TIME_LIMIT {
            let v = self.generate_random();
            let score = self.calc_scores(v);
            if score > best_score {
                best_score = score;
                best = v;
            }
            ctr += 1;
        }
        eprintln!("Loop : {}", ctr);
        eprintln!("Score: {}", best_score);
        best
    }

    pub fn calc_scores(&mut self, t: [usize; D]) -> i64 {
        let mut score = 0;
        self.last = [0i64; TYPES];
        for d in 0..D {
            let ty = t[d];
            score += self.sat[d][ty];
            self.last[ty] = d as i64 + 1;
            for i in 0..TYPES {
                score -= self.decay_factor[i] * (d as i64 + 1 - self.last[i]);
            }
        }
        score
    }

    pub fn diff_scores(&mut self, t: [usize; D], d: usize, ty: usize) -> i64 {
        let mut diff = 0;
        let (ty_before, ty_after) = (t[d], ty);
        // d日目ty_beforeをなかったことに
        diff -= self.sat[d][ty_before];
        diff += self.decay_factor[ty_before];
        // d日目ty_afterを代わりに実行する
        diff += self.sat[d][ty_after];
        diff -= self.decay_factor[ty_after];
        diff
    }
}

#[fastout]
fn main() {
    input!(d: usize);
    assert_eq!(d, D);
    let mut c = [0i64; TYPES];
    let mut s = [[0i64; TYPES]; D];
    for i in 0..TYPES {
        input!(x: i64);
        c[i] = x;
    }
    for i in 0..D {
        for j in 0..TYPES {
            input!(x: i64);
            s[i][j] = x;
        }
    }
    let mut solver = Solver::new(c, s);

    let ans = solver.solve_random();

    for i in 0..D {
        println!("{}", ans[i] + 1);
    }
}
