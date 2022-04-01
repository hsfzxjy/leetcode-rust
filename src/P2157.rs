use std::collections::HashMap;

#[inline(always)]
fn word_to_bits(s: String) -> u32 {
    let mut x = 0u32;
    for c in s.into_bytes() {
        x |= 1 << (c - b'a') as u32;
    }
    x
}

const INVALID: usize = usize::MAX / 2;

fn dfs(
    id: usize,
    counter: &mut usize,
    max_size: &mut usize,
    connects: &mut [usize],
    counts: &mut [usize],
) -> usize {
    let link = connects[id];
    if link > INVALID {
        usize::MAX - link
    } else if link == id {
        *counter += 1;
        *max_size = (*max_size).max(counts[id]);
        connects[id] = usize::MAX - id;
        id
    } else {
        let newid = dfs(link, counter, max_size, connects, counts);
        counts[newid] += counts[id];
        *max_size = counts[newid].max(*max_size);
        connects[id] = usize::MAX - newid;
        newid
    }
}

#[inline(always)]
fn process_word(
    w: u32,
    ht: &HashMap<u32, usize>,
    connects: &mut [usize],
    maxid: &mut Option<usize>,
) {
    let mut y = w;
    if let Some(id) = ht.get(&w) {
        handle_matched(*id, maxid, connects);
    }
    while y != 0 {
        let x = y ^ (y - 1);
        let x = x ^ (x >> 1);
        if let Some(id) = ht.get(&(w - x)) {
            handle_matched(*id, maxid, connects);
        };
        y -= x;
    }
}

#[inline(always)]
fn merge(connects: &mut [usize], mut x: usize, mut y: usize) -> usize {
    while x != connects[x] {
        x = connects[x]
    }
    while y != connects[y] {
        y = connects[y]
    }
    if x < y {
        connects[x] = y;
        y
    } else {
        connects[y] = x;
        x
    }
}

#[inline(always)]
fn handle_matched(res: usize, maxid: &mut Option<usize>, connects: &mut [usize]) {
    match maxid {
        y @ None => {
            y.replace(res);
        }
        Some(y) => {
            *y = merge(connects, res, *y).max(*y);
        }
    };
}

impl Solution {
    pub fn group_strings(mut words: Vec<String>) -> Vec<i32> {
        let mut ht = HashMap::<u32, usize>::new();
        let mut counts = vec![];
        let mut connects = vec![];
        let mut counter = 0;
        words.sort_unstable_by_key(|x| x.len());
        for word in words {
            let word = word_to_bits(word);
            let id = {
                let mut maxid = None;
                process_word(word, &ht, &mut connects, &mut maxid);
                maxid
            };
            let id = match id {
                None => {
                    let newid = counter;
                    counter += 1;
                    counts.push(1);
                    connects.push(newid);
                    newid
                }
                Some(x) => {
                    counts[x] += 1;
                    x
                }
            };
            {
                let mut y = word;
                ht.insert(word, id);
                while y != 0 {
                    let x = y ^ (y - 1);
                    let x = x ^ (x >> 1);
                    ht.insert(word - x, id);
                    y -= x;
                }
            }
        }
        counter = 0;
        let mut max_size = 0;
        for id in 0..connects.len() {
            dfs(id, &mut counter, &mut max_size, &mut connects, &mut counts);
        }
        vec![counter as i32, max_size as i32]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test1() {
        let input = vec!["a", "b", "ab", "cde"]
            .into_iter()
            .map(String::from)
            .collect();
        let ret = Solution::group_strings(input);
        assert_eq!(ret, vec![2, 3]);
    }

    #[test]
    fn test2() {
        let input = vec![
            "ghnv", "uip", "tenv", "hvepx", "e", "ktc", "byjdt", "ulm", "cae", "ea",
        ]
        .into_iter()
        .map(String::from)
        .collect();
        let ret = Solution::group_strings(input);
        assert_eq!(ret, vec![8, 3]);
    }
}
