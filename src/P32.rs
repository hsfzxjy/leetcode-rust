impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut max = 0;
        let s = s.into_bytes();
        if s.len() <= 1 {
            return 0;
        }
        let mut dp = vec![0i32; s.len() + 1];
        dp[0] = 0;
        for (i, c) in s[1..].iter().enumerate() {
            let i = i + 1;
            let c = *c;
            if c == b'(' {
                continue;
            }

            let mut x = 0;
            let prev = dp[i - 1];
            let y = i as i32 - 1 - prev;
            if y >= 0 && s[y as usize] == b'(' {
                x = prev + 2;
                if y > 0 {
                    x += dp[y as usize - 1]
                }
            }
            dp[i] = x;
            max = max.max(x);
        }
        max
    }
}

struct Solution;
