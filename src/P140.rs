use std::collections::HashSet;
use std::iter::FromIterator;

fn dfs<'a>(
    s: &'a [u8],
    dict: &'a HashSet<&'a [u8]>,
    mut ans: Vec<&'a [u8]>,
    mut anslist: Vec<String>,
) -> (Vec<&'a [u8]>, Vec<String>) {
    if s.is_empty() {
        anslist.push(String::from_utf8_lossy(&ans.join(&(' ' as u8))).into_owned());
        return (ans, anslist);
    }
    for i in 0..s.len() + 1 {
        let (front, tail) = s.split_at(i);
        if !dict.contains(front) {
            continue;
        }
        ans.push(front);
        let ret = dfs(tail, dict, ans, anslist);
        ans = ret.0;
        anslist = ret.1;
        ans.pop();
    }
    (ans, anslist)
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let dict = HashSet::<&[u8]>::from_iter(word_dict.iter().map(String::as_bytes));
        let anslst = vec![];
        let ans = vec![];
        let (_, anslst) = dfs(s.as_bytes(), &dict, ans, anslst);
        anslst
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::{HashSet, Solution};

    macro_rules! make_test {
        ($name: ident, $s: expr, $dict: tt, $res: tt) => {
            #[test]
            fn $name() {
                assert_eq!(
                    HashSet::<String>::from_iter(Solution::word_break(
                        $s.into(),
                        $dict.into_iter().map(String::from).collect()
                    )),
                    HashSet::from_iter($res.into_iter().map(String::from))
                )
            }
        };
    }

    make_test!(
        test1,
        "catsanddog",
        ["cat", "cats", "and", "sand", "dog"],
        ["cats and dog", "cat sand dog"]
    );
}
