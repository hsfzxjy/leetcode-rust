impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len() / 2;
        if n == 1 {
            return (nums[0] - nums[1]).abs();
        }
        let (left, right) = nums.split_at(n);
        let sl: i32 = left.iter().sum();
        let sr: i32 = right.iter().sum();
        let mut left_sums: Vec<Vec<i32>> = vec![vec![]; n + 1];
        let mut right_sums: Vec<Vec<i32>> = vec![vec![]; n + 1];

        for i in 1..(1 << n) {
            let mut cnt = 0;
            let mut tsl = 0;
            let mut tsr = 0;

            for j in 0..n {
                if (1 << j) & i == 0 {
                    continue;
                }
                cnt += 1;
                tsl += unsafe { left.get_unchecked(j) };
                tsr += unsafe { right.get_unchecked(j) };
            }

            let l = 2 * tsl - sl;
            unsafe { left_sums.get_unchecked_mut(cnt) }.push(l);

            let r = 2 * tsr - sr;
            unsafe { right_sums.get_unchecked_mut(cnt) }.push(r);
        }

        for x in &mut left_sums {
            x.sort()
        }

        for x in &mut right_sums {
            x.sort()
        }

        let mut res = (sl - sr).abs();
        for i in 1..n {
            let rs = &right_sums[n - i];
            for l in &left_sums[i] {
                let x = find_closest(rs, *l);
                if x == 0 {
                    return 0;
                }
                res = res.min(x.abs())
            }
        }

        res
    }
}

fn find_closest(nums: &[i32], num: i32) -> i32 {
    let L = nums.len();
    let mut low = 0;
    let mut high = L;

    while low <= high {
        let mid = (low + high) / 2;
        let x = if mid == 0 {
            None
        } else {
            Some(num + nums[mid - 1])
        };
        let y = if mid == L {
            None
        } else {
            Some(num + nums[mid])
        };
        use std::cmp::Ordering::*;
        let cmp = match (x.as_ref().map(|x| x.cmp(&0)), y.as_ref().map(|y| y.cmp(&0))) {
            (None, None) => unreachable!(),
            (_, Some(Equal)) => return 0,
            (Some(Equal), _) => return 0,
            (None, Some(z)) => match z {
                Greater => return y.unwrap(),
                Less => z,
                _ => unreachable!(),
            },
            (Some(z), None) => match z {
                Greater => z,
                Less => return x.unwrap(),
                _ => unreachable!(),
            },
            (Some(Less), Some(Greater)) => return x.unwrap().abs().min(y.unwrap().abs()),
            (_, Some(Less)) => Less,
            (Some(Greater), _) => Greater,
        };
        match cmp {
            Less => low = mid + 1,
            Greater => high = mid,
            _ => unreachable!(),
        }
    }
    unreachable!()
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        let inputs = vec![2, -1, 0, 4, -2, -9];
        assert_eq!(Solution::minimum_difference(inputs), 0);
    }

    #[test]
    fn test2() {
        let inputs = vec![42, 41, 59, 43, 69, 67];
        assert_eq!(Solution::minimum_difference(inputs), 13);
    }
}
