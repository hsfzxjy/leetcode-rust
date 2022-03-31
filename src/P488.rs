// TLE. Should work in release mode

struct Solution;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Balls(u64);

impl std::fmt::Display for Balls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:064b} [", self.0)?;
        for i in 0..22 {
            let c = self.get(i);
            if c == 0 {
                break;
            }
            write!(f, " {}", c)?;
        }
        write!(f, " ]")
    }
}

impl Balls {
    #[inline]
    fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    fn set(&mut self, i: i8, c: u8) {
        self.0 &= !(0b111 << (i * 3));
        self.0 += (c as u64) << (i * 3);
    }

    #[inline]
    fn get(&self, i: i8) -> u8 {
        ((self.0 & (0b111 << (i * 3))) >> (i * 3)) as u8
    }

    #[inline]
    fn insert(&self, i: i8, c: u8) -> Self {
        let prefix = self.0 & ((1 << (3 * i)) - 1);
        let suffix = self.0 - prefix;
        let mut other = prefix;
        other += (c as u64) << (3 * i);
        other += suffix << 3;
        Self(other)
    }

    #[inline]
    fn reduce(&self) -> Self {
        let mut cur = Self(0);
        let mut prev = self.clone();
        loop {
            let mut reduced = false;
            let mut i = 0;
            let mut j: i8 = 0;
            while i < 22 {
                let c = prev.get(i);
                if c == 0 {
                    break;
                }
                let mut k = i + 1;
                while k < 22 && prev.get(k) == c {
                    k += 1
                }
                let span = k - i;
                i = k;
                if span >= 3 {
                    reduced = true
                } else {
                    for _ in 0..span {
                        cur.set(j, c);
                        j += 1;
                    }
                }
            }
            if !reduced {
                return cur;
            }
            prev = cur;
            cur = Self(0);
        }
    }

    fn new(s: String) -> Self {
        let mut this = Self(0);
        for (i, c) in s.into_bytes().into_iter().map(char_to_num).enumerate() {
            this.set(i as i8, c);
        }
        this
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
struct Hand(u16);

impl Hand {
    fn new(s: String) -> Self {
        let mut this = Self(0);
        for c in s.into_bytes().into_iter().map(char_to_num) {
            this.0 += 1 << (3 * (c - 1));
        }
        this
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.0 == 0
    }
    fn iter(&self) -> IterHand {
        IterHand {
            hand: self.clone(),
            i: 0,
        }
    }
}

struct IterHand {
    hand: Hand,
    i: u8,
}

impl Iterator for IterHand {
    type Item = (u8, Hand);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.i < 5 {
            let offset = self.i * 3;
            let count = self.hand.0 & (0b111 << offset);
            self.i += 1;
            if count != 0 {
                return Some((self.i, Hand(self.hand.0 - (1 << offset))));
            }
        }
        None
    }
}

type Cache = std::collections::HashSet<(Balls, Hand)>;

fn solve(balls: Balls, hand: Hand, cache: &mut Cache, count: i32, res: &mut Option<i32>) {
    if balls.is_empty() {
        if res.unwrap_or(6) > count {
            res.replace(count);
        }
        return;
    }
    if hand.is_empty() {
        return;
    }
    if count >= res.unwrap_or(6) {
        return;
    }
    if let Some(_) = cache.get(&(balls, hand)) {
        return;
    }
    for (c, new_hand) in hand.iter() {
        for i in 0..22 {
            let cc = balls.get(i);
            let mut new_balls = balls.insert(i, c);
            new_balls = new_balls.reduce();
            // println!("{} {}\n{}\n{}\n", c, i, balls, new_balls);
            solve(new_balls, new_hand, cache, count + 1, res);
            if cc == 0 {
                break;
            }
        }
    }
    cache.insert((balls, hand));
}

fn char_to_num(c: u8) -> u8 {
    match char::from_u32(c as u32).unwrap() {
        'W' => 1,
        'B' => 2,
        'G' => 3,
        'Y' => 4,
        'R' => 5,
        _ => panic!(),
    }
}

impl Solution {
    pub fn find_min_step(board: String, hand: String) -> i32 {
        let mut cache = Cache::new();
        let mut ret = None;
        solve(Balls::new(board), Hand::new(hand), &mut cache, 0, &mut ret);
        ret.unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    macro_rules! make_test {
        ($name: ident, $board: expr, $hand: expr, $res: expr) => {
            #[test]
            fn $name() {
                assert_eq!(Solution::find_min_step($board.into(), $hand.into()), $res);
            }
        };
    }

    make_test!(test1, "WRRBBW", "RB", -1);
    make_test!(test2, "WWRRBBWW", "WRBRW", 2);
    make_test!(test3, "RRWWRRBBRR", "WB", 2);
    make_test!(test4, "WWBBWBBWW", "BB", -1);
    make_test!(test5, "RRGGBBYYWWRRGGBB", "RGBYW", -1);
    make_test!(test6, "GWRBGYWGWGWYGRYW", "BRGGW", -1);
}
