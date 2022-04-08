impl Solution {
    pub fn sub_str_hash(s: String, power: i32, modulo: i32, k: i32, hash_value: i32) -> String {
        let bytes = s.as_bytes();
        let power = power as u64;
        let modulo = modulo as u64;
        let hash_value = hash_value as u64;
        let k = k as usize;
        let L = s.len();

        if k == L {
            return s;
        }

        let mut x = 0;
        let mut p = 1;
        let mut q = 1;
        for i in L - k..L {
            let c = *unsafe { bytes.get_unchecked(i) };
            p = q;
            x = (x + (c - b'a' + 1) as u64 * p) % modulo;
            q = q * power % modulo;
        }

        let mut start = if x == hash_value { L - k } else { 0 };
        for j in (0..L - k).rev() {
            let head = unsafe { bytes.get_unchecked(j) - b'a' + 1 } as u64;
            let tail = unsafe { bytes.get_unchecked(j + k) - b'a' + 1 } as u64;
            x = ((x + modulo - p * tail % modulo) * power + head) % modulo;
            if x == hash_value {
                start = j
            }
        }

        s[start..start + k].to_string()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test1() {
        let s= "bzzrtrrpppigevriaooetwawtnfwddgdvoldxucsbyaufhygdxpnxupmvwbryzlgiuierypzqvwiywqlwiygyj".into();
        let power = 76;
        let modulo = 4;
        let k = 60;
        let hash_value = 2;
        let ret = Solution::sub_str_hash(s, power, modulo, k, hash_value);
        assert_eq!(
            ret,
            "bzzrtrrpppigevriaooetwawtnfwddgdvoldxucsbyaufhygdxpnxupmvwbr"
        );
    }
}
