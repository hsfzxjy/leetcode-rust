use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

macro_rules! bor {
    ($node: expr) => {
        $node.as_ref().unwrap().borrow()
    };
    (mut $node: expr) => {
        $node.as_ref().unwrap().borrow_mut()
    };
}

fn dfs(
    id: usize,
    edges: &HashMap<usize, Vec<usize>>,
    trees: &Vec<Option<Rc<RefCell<TreeNode>>>>,
    visited: &mut HashSet<usize>,
) {
    if visited.contains(&id) {
        return;
    }
    visited.insert(id);

    let mut node = bor!(mut trees[id]);
    let val = node.val;
    if let Some(succs) = edges.get(&id) {
        for succid in succs {
            if visited.contains(succid) {
                continue;
            }
            let succ = bor!(trees[*succid]);
            let sval = succ.val;
            let slot = if sval < val {
                &mut node.left
            } else {
                &mut node.right
            };
            *slot = (trees[*succid]).clone();
            drop(succ);
            dfs(*succid, edges, trees, visited);
        }
    }
}

fn check(node: &Option<Rc<RefCell<TreeNode>>>, ok: &mut bool) -> Option<(i32, i32)> {
    if node.is_none() {
        return None;
    }
    let node = bor!(node);
    let val = node.val;
    let ret = check(&node.left, ok);
    if !*ok {
        return None;
    }
    let mut minv = val;
    let mut maxv = val;
    if let Some((min, max)) = ret {
        minv = min;
        if max >= val {
            *ok = false;
            return None;
        }
    }
    let ret = check(&node.right, ok);
    if !*ok {
        return None;
    }
    if let Some((min, max)) = ret {
        maxv = max;
        if min <= val {
            *ok = false;
            return None;
        }
    }
    Some((minv, maxv))
}

impl Solution {
    pub fn can_merge(trees: Vec<Option<Rc<RefCell<TreeNode>>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root2loc = HashMap::<i32, Vec<usize>>::new();
        for (i, node) in trees.iter().enumerate() {
            let node = bor!(node);

            for child in node.left.iter().chain(node.right.iter()) {
                let val = child.borrow().val;
                root2loc.entry(val).or_insert_with(Vec::new).push(i);
            }
        }
        let mut edges = HashMap::<usize, Vec<usize>>::new();
        let mut root_id = 0;
        let mut root_indeg = usize::MAX;
        for (i, node) in trees.iter().enumerate() {
            let node = bor!(node);
            let val = node.val;
            let indeg = match root2loc.get(&val) {
                None => 0,
                Some(loc) => {
                    let mut deg = loc.len();
                    for pid in loc {
                        let pval = bor!(trees[*pid]).val;
                        if pval > val && node.right.is_some() && bor!(node.right).val >= pval
                            || pval < val && node.left.is_some() && bor!(node.left).val <= pval
                        {
                            deg -= 1;
                            continue;
                        }
                        edges.entry(*pid).or_insert_with(Vec::new).push(i);
                    }
                    deg
                }
            };
            if root_indeg >= indeg {
                if root_indeg == 0 {
                    return None;
                }
                root_id = i;
                root_indeg = indeg;
            }
        }
        dfs(root_id, &edges, &trees, &mut Default::default());
        let root = trees[root_id].clone();
        let mut ok = true;
        check(&root, &mut ok);
        if ok {
            root
        } else {
            None
        }
    }
}

struct Solution;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{RefCell, Solution, TreeNode};

    macro_rules! node {
        ($val: expr) => {
            Some(RefCell::new(TreeNode::new($val)).into())
        };
        {$val: expr, $left: expr, $right: expr} => {
            Some(RefCell::new(TreeNode{val:$val,left:$left,right:$right}).into())
        }
    }

    macro_rules! make_test {
        ($name: ident, $trees: tt, $expected: tt) => {
            #[test]
            fn $name() {
                let trees = vec!$trees;
                let expected = $expected;
                let ret = Solution::can_merge(trees);
                assert_eq!(ret, expected);
            }
        };
    }

    make_test!(
        test1,
        [
            node! {
                 5,
                 node!(3),
                 node!(8)
            },
            node! {
                3,
                node!(2),
                node!(6)
            },
        ],
        None
    );

    make_test!(
        test2,
        [
            node! {
                 10,
                 node!(9),
                 None
            },
            node! {
                9,
                node!(8),
                None
            },
            node! {
                8,
                node!(7),
                None
            }
        ],
        {
            node! {
                10,
                node! {
                    9,
                    node! {
                        8,
                        node!(7),
                        None
                    },
                    None
                },
                None
            }
        }
    );

    make_test!(
        test3,
        [
            node! {
                1,
                None,
                node!(2)
            },
            node! {
                3,
                node!(1),
                None
            },
            node! {
                2,
                None,
                node!(3)
            }
        ],
        None
    );

    make_test!(
        test4,
        [
            node! {
                2,
                node!(1),
                None
            },
            node! {
                3,
                node!(2),
                node!(5)
            },
            node! {
                5,
                node!(4),
                None
            }
        ],
        {
            node! {
                3,
                node! {
                    2,
                    node!(1),
                    None
                },
                node! {
                    5,
                    node!(4),
                    None
                }
            }
        }
    );
}
