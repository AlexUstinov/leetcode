pub struct Solution;

impl Solution {
    pub fn minimum_score(s: String, t: String) -> i32 {
        fn check(gap: usize, fwd: &[Option<usize>], bwd: &[Option<usize>]) -> bool {
            gap == fwd.len() || fwd[fwd.len() - gap - 1].is_some() || bwd[gap].is_some()
                || fwd.iter().zip(bwd.iter().skip(gap + 1))
                    .filter_map(|(opt_l, opt_r)| opt_l.zip(*opt_r))
                    .any(|(l, r)| l < r)
        }
        let n = t.len();
        let (mut fwd, mut bwd) = (vec![None; n], vec![None; n]);
        fn find_match(s_iter: &mut impl Iterator<Item=(usize, u8)>, t_char: u8) -> Option<usize> {
            s_iter.find(|&(_, s_char)| t_char==s_char).map(|(s_idx, _)| s_idx)
        }
        for (idx, fwd_match) in t.bytes().scan(s.bytes().enumerate(), find_match).enumerate() {
            fwd[idx] = Some(fwd_match);
        }
        for (idx, bwd_match) in t.bytes().rev().scan(s.bytes().enumerate().rev(), find_match).enumerate() {
            bwd[n - idx - 1] = Some(bwd_match);
        }

        let (mut lo, mut hi) = (0, t.len());
        while lo < hi {
            let mid = (lo + hi) / 2;
            if check(mid, &fwd[..], &bwd[..]) {
                hi = mid;
            }
            else {
                lo = mid + 1;
            }
        }
        lo as i32
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;

    #[test_case("cde", "xyz", 3)]
    #[test_case("abacaba", "bzaa", 1)]
    fn solve(s: &str, t: &str, expected: i32) {
        assert_eq!(expected, Solution::minimum_score(String::from(s), String::from(t)));
    }

}