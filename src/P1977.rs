const MOD: i64 = 1_000_000_007;

#[inline(always)]
fn no_greater_than(s1: &[u8], s2: &[u8]) -> bool {
    if s1.len() > s1.len() {
        return false;
    }
    for (c1, c2) in s1.iter().zip(s2.iter()) {
        if c1 < c2 {
            return true;
        } else if c1 > c2 {
            return false;
        }
    }
    return true;
}

macro_rules! get {
    ($arr: ident, $i: expr) => {
        unsafe { $arr.get_unchecked($i) }
    };
    ($arr: ident, $i: expr, $j: expr) => {
        unsafe { $arr.get_unchecked($i).get_unchecked($j) }
    };
    (mut $arr: ident, $i: expr, $j: expr) => {
        unsafe { $arr.get_unchecked_mut($i).get_unchecked_mut($j) }
    };
}

impl Solution {
    pub fn number_of_combinations(num: String) -> i32 {
        let num = num.as_bytes();
        let n = num.len();
        if n == 1 {
            return if *get!(num, 0) == b'0' { 0 } else { 1 };
        }
        let mut dp = Vec::<Vec<i64>>::with_capacity(n + 3);

        for i in 0..=n {
            dp.push(vec![0; n - i + 1]);
        }

        dp[n - 1][1] = if *get!(num, n - 1) == b'0' { 0 } else { 1 };
        for i in (0..n - 1).rev() {
            let mut r = 0i64;
            if *get!(num, i) == b'0' {
                continue;
            }
            for j in i + 1..=n {
                let m = j - i;
                let mut k = j + m;
                if j == n {
                    r += 1;
                } else if *get!(num, j) != b'0' {
                    if k <= n {
                        if !no_greater_than(&num[i..j], &num[j..k]) {
                            k += 1;
                        }
                    }
                    if k <= n {
                        r = (r + *get!(dp, j, n - j) + MOD - *get!(dp, j, k - j - 1)) % MOD;
                    }
                }
                *get!(mut dp, i, j - i) = r;
            }
        }
        return dp[0][n] as i32;
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        let s = [b'1'; 3500];
        let num = String::from_utf8_lossy(&s).to_string();
        assert_eq!(Solution::number_of_combinations(num), 1);
    }
}
