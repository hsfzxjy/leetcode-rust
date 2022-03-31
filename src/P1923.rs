// rolling hash
use std::collections::HashSet;

struct RabinKarp {
    pows: Vec<i128>,
    vals: Vec<i128>,
}

const MOD: i128 = 1_111_111_111_111_111_111;
const POW: i128 = 1_000_000_007;

impl RabinKarp {
    fn new(vals: &[i32]) -> Self {
        let n = vals.len();
        let mut this = Self {
            pows: Vec::with_capacity(n + 2),
            vals: Vec::with_capacity(n + 2),
        };

        let mut lastp = 1;
        let mut lastv = 0;
        this.pows.push(lastp);
        this.vals.push(lastv);

        for x in vals {
            let x = *x as i128;
            lastp = (lastp * POW) % MOD;
            this.pows.push(lastp);
            lastv = (lastv * POW + x) % MOD;
            this.vals.push(lastv);
        }

        this
    }

    #[inline(always)]
    fn query(&self, i: usize, j: usize) -> i128 {
        (self.vals[j] + MOD - self.vals[i] * self.pows[j - i] % MOD) % MOD
    }

    #[inline(always)]
    fn query_all(&self, span: usize) -> HashSet<i128> {
        let n = self.pows.len() - 1;
        let mut cache = HashSet::new();
        if span == 0 {
            return cache;
        }
        for i in 0..n - span + 1 {
            let hash = self.query(i, i + span);
            cache.insert(hash);
        }
        cache
    }
}

impl Solution {
    pub fn longest_common_subpath(n: i32, paths: Vec<Vec<i32>>) -> i32 {
        let m = paths.len();
        if m == 0 {
            return 0;
        } else if m == 1 {
            return paths[0].len() as i32;
        }
        let mut minl = 1_000_000;
        let mut rks = Vec::with_capacity(n as usize);
        for path in paths {
            minl = minl.min(path.len());
            if minl == 0 {
                return 0;
            }
            rks.push(RabinKarp::new(&path));
        }
        let mut l = 0;
        let mut r = minl;
        while l < r {
            let m = (l + r + 1) / 2;
            let mut cache = rks[0].query_all(m);
            for rk in &rks[1..] {
                let other = rk.query_all(m);
                cache = cache.intersection(&other).cloned().collect();
                if cache.len() == 0 {
                    break;
                }
            }
            if cache.len() == 0 {
                r = m - 1;
            } else {
                l = m
            }
        }
        l as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let n = 5;
        let input = vec![vec![0, 1, 2, 3, 4], vec![2, 3, 4], vec![4, 0, 1, 2, 3]];
        assert_eq!(super::Solution::longest_common_subpath(n, input), 2);
    }
}
