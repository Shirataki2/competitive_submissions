use proconio::{input, fastout};
use std::collections::HashSet;

fn gcd(a: usize, b: usize) -> usize {
    match (a, b) {
        (a, 0) => a,
        (a, b) => gcd(b, a % b)
    }
}

#[derive(Debug)]
struct SieveFactorizer
{
    minimum: usize,
    maximum: usize,
    max_sqrt: usize,
    small: Vec<usize>,
    large: Vec<Vec<usize>>,
    aux: Vec<usize>,
}

impl SieveFactorizer
{
    fn new(minimum: usize, maximum: usize) -> SieveFactorizer {
        let max_sqrt = (maximum as f64).sqrt() as usize + 1;
        SieveFactorizer {
            minimum: minimum,
            maximum: maximum,
            max_sqrt: max_sqrt,
            small: (0..max_sqrt).map(|i| i).collect::<Vec<usize>>(),
            large: vec![Vec::new(); maximum - minimum],
            aux: vec![1; maximum - minimum]
        }
    }

    fn build(&mut self) -> () {
        for i in 2..self.maximum {
            if i * i >= self.maximum {
                break;
            }
            if self.small[i] < i {
                continue;
            }
            self.small[i] = i;
            let mut j = i * i;
            while j < self.max_sqrt {
                if self.small[j] == j {
                    self.small[j] = i;
                }
                j += i;
            }
            let mut j = (self.minimum + i - 1) / i * i;
            while j < self.maximum {
                let mut k = j;
                while {
                    if self.aux[j-self.minimum] * self.aux[j-self.minimum] > self.maximum {
                        false
                    } else {
                        self.large[j-self.minimum].push(i);
                        self.aux[j-self.minimum] *= i;
                        k /= i;
                        k % i == 0
                    } 
                } {}
                j += i;
            }
        }
    }

    fn factor(&self, n: usize) -> Vec<usize> {
        assert!(self.minimum <= n && n < self.maximum);
        if n == 1 { return vec![1] }
        let mut m = n;
        let mut res = self.large[n - self.minimum].clone();
        m /= self.aux[n - self.minimum];
        if m >= self.max_sqrt {
            res.push(m);
        } else {
            while m > 1 {
                res.push(self.small[m]);
                m /= self.small[m];
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_factor_correct() {
        let mut sf = SieveFactorizer::new(1, 1000);
        sf.build();
        let res = sf.factor(50);
        assert_eq!(res, vec![2, 5, 5]);
        let res = sf.factor(8);
        assert_eq!(res, vec![2, 2, 2]);
        let res = sf.factor(1);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn should_factor_large_number() {
        let mut sf = SieveFactorizer::new(123_000_000, 124_000_000);
        sf.build();
        let res = sf.factor(123_456_789);
        assert_eq!(res, vec![3, 3, 3607, 3803]);
    }
}

#[fastout]
fn main() {
    input!(n: usize, a: [usize; n]);
    let mut sf = SieveFactorizer::new(1, 1010000);
    sf.build();
    let mut map = vec![0; 1010000];
    let mut f = true;
    for ai in a.iter() {
        let fac = sf.factor(*ai);
        let fac: HashSet<usize> = fac.into_iter().collect();
        for e in fac {
            map[e] += 1;
            if map[e] >= 2 && e != 1 {
                f = false;
            }
        }
    }
    if f {
        println!("pairwise coprime");
        return
    }
    println!("{}", 
        match a.iter().fold(a[0], |acc, x| gcd(acc, *x)) {
            1 => "setwise coprime",
            _ => "not coprime"
    });
}
