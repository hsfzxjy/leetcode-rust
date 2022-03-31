fn gcd(mut x: i64, mut y: i64) -> i64 {
    if x < y {
        std::mem::swap(&mut x, &mut y);
    }
    while y != 0 {
        let t = x % y;
        x = y;
        y = t;
    }
    x
}

const MOD: i64 = 1000_000_007;

impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        let n = n as i64;
        let a = a as i64;
        let b = b as i64;

        let d = gcd(a, b);
        let lcm = a / d * b;
        let mut low = 1;
        let mut high = 1_000_000_000_000_000i64;

        while low < high {
            let mid = (low + high) / 2;
            let pos = mid / a + mid / b - mid / lcm;
            if pos < n {
                low = mid + 1
            } else {
                high = mid
            }
        }

        (low % MOD) as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    macro_rules! make_test {
        ($name: ident, $n: expr, $a: expr, $b: expr, $res: expr) => {
            #[test]
            fn $name() {
                assert_eq!(Solution::nth_magical_number($n, $a, $b), $res);
            }
        };
    }

    make_test!(test1, 1, 2, 3, 2);
    make_test!(test2, 4, 2, 3, 6);
    make_test!(test3, 3, 6, 4, 8);
    make_test!(test4, 3, 8, 3, 8);
    make_test!(test5, 12086002, 23434, 22837, 787605335);
}
