struct Vec2D<T> {
    inner: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Default + Clone> Vec2D<T> {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            inner: vec![Default::default(); rows * cols],
            rows,
            cols,
        }
    }
    #[inline(always)]
    fn get(&self, i: usize, j: usize) -> &T {
        unsafe { self.inner.get_unchecked(i * self.cols + j) }
    }
    #[inline(always)]
    fn get_mut(&mut self, i: usize, j: usize) -> &mut T {
        unsafe { self.inner.get_unchecked_mut(i * self.cols + j) }
    }
}

fn do_match<'a>(s: &[u8], p: &[u8], i: usize, j: usize, dp: &mut Vec2D<Option<bool>>) -> bool {
    use std::cmp::Ordering::*;

    if let Some(x) = dp.get(i, j) {
        return *x;
    }

    match (i.cmp(&s.len()), j.cmp(&p.len())) {
        (Equal, Equal) => {
            dp.get_mut(i, j).replace(true);
            return true;
        }
        (_, Equal) => {
            dp.get_mut(i, j).replace(false);
            return false;
        }
        (Equal, _) => {
            let mut res = true;
            for cp in &p[j..] {
                if *cp != b'*' {
                    res = false;
                    break;
                }
            }
            dp.get_mut(i, j).replace(res);
            return res;
        }
        _ => {}
    };

    let res = match p[j] {
        pc @ b'a'..=b'z' => pc == s[i] && do_match(s, p, i + 1, j + 1, dp),
        b'*' => {
            do_match(s, p, i, j + 1, dp)
                || do_match(s, p, i + 1, j , dp)
        }
        b'?' => do_match(s, p, i + 1, j + 1, dp),
        _ => unreachable!(),
    };

    dp.get_mut(i, j).replace(res);
    res
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.into_bytes();
        let p = p.into_bytes();

        do_match(
            &s,
            &p,
            0,
            0,
            &mut Vec2D::<Option<bool>>::new(s.len() + 1, p.len() + 1),
        )
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_match("aa".into(), "*".into()), true)
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::is_match("aa".into(), "a".into()), false)
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::is_match("adceb".into(), "*a*b".into()), true)
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::is_match("acdcb".into(), "a*c?b".into()), false)
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::is_match("mississippi".into(), "m??*ss*?i*pi".into()),
            false
        )
    }

    #[test]
    fn test6() {
        assert_eq!(
            Solution::is_match("abcdefghij".into(), "abc*defghijk".into()),
            false
        )
    }

    #[test]
    fn test7() {
        let s = "abbabaaabbabbaababbabbbbbabbbabbbabaaaaababababbbabababaabbababaabbbbbbaaaabababbbaabbbbaabbbbababababbaabbaababaabbbababababbbbaaabbbbbabaaaabbababbbbaababaabbababbbbbababbbabaaaaaaaabbbbbaabaaababaaaabb";
        let p = "**aa*****ba*a*bb**aa*ab****a*aaaaaa***a*aaaa**bbabb*b*b**aaaaaaaaa*a********ba*bbb***a*ba*bb*bb**a*b*bb";
        assert_eq!(Solution::is_match(s.into(), p.into()), false)
    }
}
