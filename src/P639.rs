const MOD: i64 = 1000_000_007;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut dp0 = 1;
        let mut c0 = bytes[0];
        let mut dp1 = match c0 {
            b'*' => 9,
            b'0' => return 0,
            _ => 1,
        };
        for c1 in &bytes[1..] {
            let mut s = match c1 {
                b'*' => 9,
                b'0' => 0,
                _ => 1,
            } * dp1
                % MOD;
            let t = match (c0, c1) {
                (b'*', b'*') => 15,
                (b'*', b'0'..=b'6') => 2,
                (b'*', _) => 1,
                (b'1', b'*') => 9,
                (b'1', _) => 1,
                (b'2', b'*') => 6,
                (b'2', b'0'..=b'6') => 1,
                _ => 0,
            };
            s = (s + t * dp0) % MOD;
            dp0 = dp1;
            dp1 = s;
            c0 = *c1;
        }
        dp1 as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    macro_rules! make_test {
        ($name: ident, $s: expr, $res: tt) => {
            #[test]
            fn $name() {
                assert_eq!(Solution::num_decodings($s.into()), $res)
            }
        };
    }

    make_test!(test1, "*", 9);
    make_test!(test2, "1*", 18);
    make_test!(test3, "**", 96);
    make_test!(test4, "*1", 11);
}
