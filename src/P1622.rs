struct Operetion {
    until: usize,
    mul: i64,
    add: i64,
}

struct Fancy {
    nums: Vec<i64>,
    ops: Vec<Operetion>,
}

const MOD: i64 = 1_000_000_007;

impl Fancy {
    fn new() -> Self {
        Self {
            nums: Vec::with_capacity(5_0000),
            ops: Vec::with_capacity(5_0000),
        }
    }

    fn append(&mut self, val: i32) {
        self.nums.push(val as i64)
    }

    #[inline(always)]
    fn cur_idx(&self) -> usize {
        self.nums.len() - 1
    }

    fn add_all(&mut self, inc: i32) {
        let inc = inc as i64;
        let cur = self.cur_idx();
        if let Some(op) = self.ops.last_mut() {
            if op.until == cur {
                op.add = (op.add + inc) % MOD;
                return;
            }
        }
        self.ops.push(Operetion {
            until: cur,
            add: inc,
            mul: 1,
        })
    }

    fn mult_all(&mut self, m: i32) {
        let m = m as i64;
        let cur = self.cur_idx();
        if let Some(op) = self.ops.last_mut() {
            if op.until == cur {
                op.add = (op.add * m) % MOD;
                op.mul = (op.mul * m) % MOD;
                return;
            }
        }
        self.ops.push(Operetion {
            until: cur,
            add: 0,
            mul: m,
        })
    }

    fn get_index(&self, idx: i32) -> i32 {
        let idx = idx as usize;
        if idx >= self.nums.len() {
            return -1;
        }
        let mut x = self.nums[idx];
        match self.ops.last() {
            None => return x as i32,
            Some(Operetion { until, .. }) if *until < idx => return x as i32,
            _ => {}
        };
        let mut l = 0;
        let mut r = self.ops.len() - 1;
        while l < r {
            let m = (l + r) / 2;
            let opm = &self.ops[m];
            if opm.until < idx {
                l = m + 1
            } else {
                r = m
            };
        }
        for op in &self.ops[l..] {
            x = ((x * op.mul) + op.add) % MOD;
        }
        x as i32
    }
}
