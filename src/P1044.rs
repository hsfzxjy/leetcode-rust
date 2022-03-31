use std::collections::{HashMap, HashSet};

const MOD: i128 = 1_111_111_111_111_111_111;
const POW: i128 = 1_000_000_007;

impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        let oris = s.as_bytes();
        let s = oris.iter().map(|c| (*c - b'a') as i128).collect::<Vec<_>>();
        let L = s.len();
        let mut pows = Vec::<i128>::with_capacity(L + 2);
        let mut rolls = Vec::<i128>::with_capacity(L + 2);

        let mut lastp = 1;
        let mut lastr = 0;
        pows.push(lastp);
        rolls.push(lastr);
        for c in &s {
            lastp = (lastp * POW) % MOD;
            pows.push(lastp);
            lastr = (lastr * POW + c) % MOD;
            rolls.push(lastr);
        }

        let mut l = 1;
        let mut h = L;
        let mut ht = HashSet::<i128>::new();
        let mut res = HashMap::<usize, Option<usize>>::new();
        while l <= h {
            if l == h {
                match res.get(&l) {
                    Some(Some(j)) => return String::from_utf8_lossy(&oris[*j..*j + l]).into(),
                    Some(None) => return "".into(),
                    _ => {}
                }
            }
            let m = (l + h + 1) / 2;
            ht.clear();
            let mut hash = rolls[m];
            ht.insert(hash);
            let P = pows[m - 1];
            let mut r = None;
            for j in 1..=L - m {
                let x = unsafe { s.get_unchecked(j - 1) };
                let y = unsafe { s.get_unchecked(j + m - 1) };
                let new_hash = ((hash + MOD - P * x % MOD) % MOD * POW + y) % MOD;
                if ht.contains(&new_hash) {
                    r.replace(j);
                    break;
                }
                ht.insert(new_hash);
                hash = new_hash;
            }
            if r.is_some() {
                l = m;
            } else {
                h = m - 1
            }
            res.insert(m, r);
        }
        "".into()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_dup_substring("aa".into()), "a");
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::longest_dup_substring("yepiywfjnsbvoutspszhkjmtjzvksdoliemtctsffveambxsbwbitxwzwzaraaoofenhvfqhkkkgyowttycdtyjdivmalvgtbayvzauyqfxgrawmpdtbshtuimblnatfrncdeuxtnweiptyskfauqcpnxmfksocacsbzgjbjzlopefwyrafayyfscsmfledxcprzfuhtfvvspunmvibfozatsgtpzkfeupfchrdrxfouqjbqdgchtgeegabohgvzbjhuvigxnjbqdeonysrqpnvrfdvoxnskimpduhwdrobfrwidgvoamjrpmiqlegomwzxbwxqkpiqoxlzqmbwrvoxbwayfeprxsrizkqryclzjeztbnjcaajjbgrlzryjditssvqvrzwytsdtnrmujkdbntiystingvhhbhtibdxaxxgkakkhqyhddwsexqarxmaxnurxlvjgxjxlasvyywelmmjfebosmsngleklaychavswwisnfglzqwungllebkfonbiwyycandlhjelowdcbcxzrjpbbsvbnjxwqcqwaxciugbtiwfovrkwexjxvlohjisyhzgttgrnbbipbbqgevsqegkgjfszjpiesjzdapjqmw".into()),"jbqd");
    }
}
