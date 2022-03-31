#[derive(Debug, Copy, Clone)]
enum Pat<'a> {
    Exact(usize),
    AtLeast(usize),
    AtLeastNChar(u8, usize),
    Literal(&'a [u8]),
}
use Pat::*;

impl<'a> Pat<'a> {
    #[inline(always)]
    fn min_len(&self) -> usize {
        match self {
            Exact(n) => *n,
            AtLeast(n) => *n,
            AtLeastNChar(_, n) => *n,
            Literal(x) => x.len(),
        }
    }
}

#[inline(always)]
fn min_len<'a>(pat: Option<Pat<'a>>) -> usize {
    match pat {
        None => 0,
        Some(x) => x.min_len(),
    }
}

fn do_match(s: &[u8], pats: &[Pat]) -> bool {
    if pats.is_empty() {
        return s.is_empty();
    }
    let (curpat, restpat) = unsafe { pats.split_first().unwrap_unchecked() };
    let nextpat = restpat.get(0).cloned();
    let sl = s.len();
    match (*curpat, nextpat) {
        (Exact(n), None) => return n == sl,
        (Exact(n), _) => {
            if sl < n {
                return false;
            }
            return do_match(&s[n..], restpat);
        }
        (AtLeastNChar(ch, num), None | Some(Literal(_) | AtLeastNChar(_, 1..))) => {
            let mut cnt = 0;
            if sl < num {
                return false;
            }
            for c in s {
                if *c != ch {
                    break;
                }
                cnt += 1;
            }
            return nextpat.is_none() && cnt == sl
                || cnt >= num && cnt < sl && do_match(&s[cnt..], restpat);
        }
        (AtLeastNChar(ch, num), _) => {
            if sl < num {
                return false;
            }
            let (prefix, s) = s.split_at(num);
            for x in prefix {
                if *x != ch {
                    return false;
                }
            }
            if do_match(s, restpat) {
                return true;
            }
            let mlen = min_len(nextpat);
            for (i, c) in s.iter().enumerate() {
                if *c != ch {
                    return false;
                }
                let rest = &s[i + 1..];
                if do_match(rest, restpat) {
                    return true;
                }
                if rest.len() < mlen {
                    return false;
                }
            }
            return false;
        }
        (AtLeast(n), None) => return sl >= n,
        (AtLeast(n), _) => {
            let mlen = min_len(nextpat);
            if sl < n + mlen {
                return false;
            }
            for i in n..=sl - mlen {
                if do_match(&s[i..], restpat) {
                    return true;
                }
            }
            return false;
        }
        (Literal(lit), None) => return lit == s,
        (Literal(lit), _) => {
            if sl < lit.len() {
                return false;
            }
            let (prefix, rest) = s.split_at(lit.len());
            if prefix != lit {
                return false;
            }
            return do_match(rest, restpat);
        }
    }
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut pats = vec![];
        let mut lit_start = None;
        for (i, c) in p.iter().enumerate() {
            let c = *c;
            if lit_start.is_some() {
                match c {
                    b'.' => pats.push(Literal(&p[lit_start.take().unwrap()..i])),
                    b'*' => {
                        let start = lit_start.take().unwrap();
                        if start < i - 1 {
                            pats.push(Literal(&p[start..i - 1]));
                        }
                        pats.push(AtLeastNChar(p[i - 1], 0));
                        continue;
                    }
                    _ => continue,
                }
            }
            let mut to_add = false;
            match pats.last_mut() {
                None => to_add = true,
                Some(pat) => match pat {
                    Exact(x) => match c {
                        b'*' => *pat = AtLeast(*x - 1),
                        b'.' => *x += 1,
                        _ => to_add = true,
                    },
                    AtLeast(x) => match c {
                        b'*' => *x -= 1,
                        b'.' => *x += 1,
                        _ => to_add = true,
                    },
                    AtLeastNChar(ch, num) => match c {
                        b'*' => *num -= 1,
                        x if x == *ch => *num += 1,
                        _ => to_add = true,
                    },
                    _ => to_add = true,
                },
            }
            if to_add {
                match c {
                    b'*' => {
                        unreachable!()
                    }
                    b'.' => pats.push(Exact(1)),
                    _ => lit_start = Some(i),
                }
            }
        }
        if lit_start.is_some() {
            pats.push(Literal(&p[lit_start.take().unwrap()..]));
        }
        do_match(s, &pats)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_match("aa".into(), "a".into()), false)
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::is_match("ab".into(), ".*".into()), true)
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::is_match("aa".into(), "a*".into()), true)
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::is_match("aab".into(), "c*a*b".into()), true)
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::is_match("caabbbbcacabbabbcaa".into(), ".*.*a.*c*a*a*a*ac".into()),
            false
        )
    }

    #[test]
    fn test6() {
        assert_eq!(
            Solution::is_match("mississippi".into(), "mis*is*p*.".into()),
            false
        )
    }

    #[test]
    fn test7() {
        assert_eq!(Solution::is_match("b".into(), "b.*c".into()), false)
    }
}
