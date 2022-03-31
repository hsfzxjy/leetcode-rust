use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut word_map = HashMap::<&[u8], (isize, isize)>::new();
        for word in &words {
            let word = word.as_bytes();
            word_map.entry(word).or_insert((0, 0)).0 += 1;
        }
        let mut res = vec![];
        let count = words.len();
        let wlen = words[0].len();

        let s = s.as_bytes();
        let mut touched: Vec<*mut isize> = vec![];
        'outer: for i in 0..s.len() {
            unsafe {
                for x in &touched {
                    **x = 0;
                }
            }
            touched.clear();
            let mut j = count;
            for chunk in (&s[i..]).chunks_exact(wlen) {
                match word_map.get_mut(chunk) {
                    None => continue 'outer,
                    Some((cap, cnt)) => {
                        if cap == cnt {
                            continue 'outer;
                        } else {
                            *cnt += 1;
                            touched.push(cnt as *mut isize);
                        }
                    }
                }
                j -= 1;
                if j == 0 {
                    break;
                }
            }
            if j == 0 {
                res.push(i as i32)
            }
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::collections::HashSet;

    macro_rules! make_test {
        ($name: ident, $s: literal, $words: tt, $ans: tt) => {
            #[test]
            fn $name() {
                let s = $s;
                let words = vec!$words;
                let res = Solution::find_substring(
                    s.into(),
                    words.into_iter().map(String::from).collect(),
                );
                let res = HashSet::<i32>::from_iter(res);
                let ans = HashSet::from_iter($ans);
                assert_eq!(res, ans);
            }
        };
    }

    make_test!(
        test1,
        "barfoofoobarthefoobarman",
        ["bar", "foo", "the"],
        [6, 9, 12]
    );

    make_test!(
        test2,
        "wordgoodgoodgoodbestword",
        ["word", "good", "best", "good"],
        [8]
    );
}
