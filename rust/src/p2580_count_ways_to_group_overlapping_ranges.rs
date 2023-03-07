pub struct Solution;

struct UnionFind {
    root: Vec<usize>,
    rank: Vec<i32>
}
impl UnionFind {
    pub fn new(size: usize) -> Self {
        let mut root = Vec::with_capacity(size);
        root.extend(0..size);
        let rank = vec![0; size];
        UnionFind { root, rank }
    }
    
    pub fn get_count(&self) -> usize {
        let mut set = std::collections::HashSet::new();
        for rt in &self.root {
            set.insert(rt);
        }
        set.len()
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let (rt_a, rt_b) = (self.find(a), self.find(b));
        if rt_a != rt_b {
            if self.rank[rt_a] > self.rank[rt_b] {
                self.root[rt_b] = rt_a;
            }
            else {
                self.root[rt_a] = rt_b;
            }
            if self.rank[rt_b] == self.rank[rt_a] {
                self.rank[rt_a] += 1;
            }
        }
    }

    pub fn find(&mut self, a: usize) -> usize {
        self.root[a] = {
            let rt = self.root[a];
            if a==rt { rt } else { self.find(rt) }
        };
        self.root[a]
    }
}

impl Solution {
    pub fn count_ways(ranges: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut uf = UnionFind::new(ranges.len());
        
        for (i, r1) in ranges.iter().enumerate() {
            for (j, r2) in ranges[0..i].iter().enumerate() {
                if r1[0]<=r2[1] && r1[1] >= r2[0] {
                    uf.union(i, j);
                }
            }
        }
        
        let mut count = uf.get_count() as i32;
        let mut ans = 0i64;
        let mut pow = 2;
        while count > 0 {
            if count & 1 == 1 {
                ans = (ans * pow) % MOD; 
            }
            pow = (pow * pow) % MOD;
            count >>= 1;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;
    use crate::util::parse_pairs;

    #[test_case(parse_pairs("[[6,10],[5,15]]"), 2)]
    fn solve(ranges: Vec<Vec<i32>>, expected: i32) {
        assert_eq!(expected, Solution::count_ways(ranges));
    }

}