use std::collections::{HashMap, LinkedList};

#[inline(always)]
fn word_to_bits(word: &[u8]) -> u32 {
    let mut res = 0;
    for (i, c) in word.iter().enumerate() {
        res |= ((*c - b'a' + 1) as u32) << (i as u32 * 5);
    }
    res
}

#[inline(always)]
fn clear_loc(word: u32, loc: usize) -> u32 {
    let mask = u32::MAX - (0x1F << (loc * 5));
    word & mask
}

#[derive(Default, Debug)]
struct ResultNode {
    nexts: LinkedList<usize>,
    level: usize,
}

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        mut word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let L = word_list.len();
        let N = begin_word.len();
        let mut words = word_list
            .iter()
            .map(|s| word_to_bits(s.as_bytes()))
            .collect::<Vec<_>>();
        let mut indices = HashMap::<u32, usize>::with_capacity(L + 2);
        for (i, word) in words.iter().cloned().enumerate() {
            indices.insert(word, i);
        }
        let begin_word_ori = begin_word.clone();
        let begin_word = word_to_bits(begin_word.as_bytes());
        let end_word = word_to_bits(end_word.as_bytes());
        let begin_index = *indices.entry(begin_word).or_insert_with(|| {
            words.push(begin_word);
            word_list.push(begin_word_ori);
            word_list.len() - 1
        });
        let end_index = match indices.get(&end_word) {
            Some(x) => *x,
            None => return vec![],
        };
        let mut slots = HashMap::<u32, LinkedList<usize>>::new();
        for (word, i) in indices.iter() {
            let word = *word;
            let i = *i;
            for j in 0..N {
                let slot = clear_loc(word, j);
                slots.entry(slot).or_default().push_front(i);
            }
        }

        if (0..N).all(|j| slots.get(&clear_loc(end_word, j)).unwrap().len() == 1)
            || (0..N).all(|j| slots.get(&clear_loc(begin_word, j)).unwrap().len() == 1)
        {
            return vec![];
        }

        let mut nodes = Vec::<Option<ResultNode>>::with_capacity(L + 2);
        (0..=L + 2).for_each(|_| nodes.push(None));
        nodes[end_index].replace(ResultNode {
            nexts: Default::default(),
            level: 0,
        });
        let qcap = L + 3;
        let mut q = vec![0usize; qcap];
        let mut head = 0;
        let mut tail = 1;
        let mut min_length = usize::MAX;
        q[0] = end_index;

        while head != tail {
            let curi = q[head];
            let cur_level = nodes[curi].as_ref().unwrap().level;
            if cur_level >= min_length {
                break;
            }
            let next_level = cur_level + 1;
            head = (head + 1) % qcap;

            for j in 0..N {
                for prev_i in slots
                    .get(&clear_loc(words[curi], j))
                    .unwrap()
                    .iter()
                    .cloned()
                {
                    if prev_i == begin_index {
                        min_length = next_level;
                    }
                    if prev_i == curi {
                        continue;
                    }
                    let prev_node = &mut nodes[prev_i];
                    match prev_node {
                        x @ None => {
                            x.replace(ResultNode {
                                nexts: LinkedList::from([curi]),
                                level: next_level,
                            });
                            q[tail] = prev_i;
                            tail = (tail + 1) % qcap;
                        }
                        Some(ref mut node) if node.level == next_level => {
                            node.nexts.push_front(curi);
                        }
                        _ => continue,
                    };
                }
            }
        }

        let mut results = vec![];
        make_results(
            begin_index,
            &word_list,
            &nodes,
            &mut vec![word_list[begin_index].clone()],
            &mut results,
        );
        results
    }
}

fn make_results(
    i: usize,
    word_list: &[String],
    nodes: &[Option<ResultNode>],
    result: &mut Vec<String>,
    results: &mut Vec<Vec<String>>,
) {
    let node = nodes[i].as_ref().unwrap();
    if node.nexts.is_empty() {
        results.push(result.clone());
        return;
    }
    for nexti in node.nexts.iter().cloned() {
        result.push(word_list[nexti].clone());
        make_results(nexti, word_list, nodes, result, results);
        result.pop();
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::collections::HashSet;

    fn to_hash_set(x: Vec<Vec<String>>) -> HashSet<String> {
        HashSet::<String>::from_iter(x.into_iter().map(|x| x.join(" ")))
    }

    macro_rules! make_test {
        ($name: ident, $begin: literal, $end: literal, $list: tt, $output: tt) => {
            #[test]
            fn $name() {
                let res = Solution::find_ladders(
                    $begin.into(),
                    $end.into(),
                    Vec::from($list.map(String::from)),
                );
                let output = make_test!(@output $output);
                assert_eq!(to_hash_set(res), to_hash_set(output));
            }
        };
        (@output []) => { vec![] };
        (@output $output: tt) => {
            $output
                .map(|v| v.into_iter().map(String::from).collect::<Vec<_>>())
                .to_vec()
        }
    }

    make_test!(
        test1,
        "hit",
        "cog",
        ["hot", "dot", "dog", "lot", "log", "cog"],
        [
            ["hit", "hot", "dot", "dog", "cog"],
            ["hit", "hot", "lot", "log", "cog"],
        ]
    );

    make_test!(test2, "hit", "cog", ["hot", "dot", "dog", "lot", "log"], []);
}
