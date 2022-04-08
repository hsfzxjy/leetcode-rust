use std::collections::HashMap;

#[inline(always)]
fn gcd(mut x: i32, mut y: i32) -> i32 {
    if x < y {
        std::mem::swap(&mut x, &mut y)
    }
    let mut r;
    while y != 0 {
        r = x % y;
        x = y;
        y = r;
    }
    x
}

#[inline(always)]
fn get_slope(mut x: i32, mut y: i32) -> (i32, i32) {
    if x == 0 {
        (0, i32::MAX)
    } else if y == 0 {
        (i32::MAX, 0)
    } else {
        if x < 0 {
            x = -x;
            y = -y;
        }
        let g = gcd(x.abs(), y.abs());
        (x / g, y / g)
    }
}

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let mut freq = HashMap::<(i32, i32), i32>::new();
        let L = points.len();
        if L == 1 {
            return 1;
        }
        let mut max = 0;
        for (i, pi) in points[..L - 1].iter().enumerate() {
            for pj in &points[i + 1..] {
                let slope = get_slope(pi[0] - pj[0], pi[1] - pj[1]);
                let cur = freq.entry(slope).or_insert(1);
                *cur += 1;
                max = max.max(*cur);
            }
            freq.clear();
        }
        max
    }
}

struct Solution;
