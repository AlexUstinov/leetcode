pub struct Solution1;
pub struct Solution2;
pub struct Solution3;

impl Solution1 {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let last_idx = nums.len() - 1;
        let mut max_idx = 0usize;
        for (idx, num) in nums.iter().enumerate() {
            match idx {
                idx if idx > max_idx => return false,
                _ if max_idx >= last_idx => return true,
                idx => max_idx = max_idx.max(idx + *num as usize)
            }
        }
        true
    }
}

impl Solution2 {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        nums.len() == 1 || nums.iter().scan(0, |jump, num| {
            match num.max(jump) - 1 {
                d if d >= 0 => { *jump = d; Some(num) },
                _ => None
            }
        }).count() == nums.len()
    }
}

impl Solution3 {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        nums.len() == 1 || nums.iter().map(|n| *n as usize).enumerate().scan(0usize, |jump, (idx, num)| {
            match num.max(*jump) {
                dist if dist > 0 => { *jump = dist - 1; Some(idx + dist + 1) },
                _ => None
            }
        }).any(|l| l >= nums.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![0], true)]
    #[test_case(vec![2,3,1,1,4], true)]
    #[test_case(vec![3,2,1,0,4], false)]
    fn solve1(nums: Vec<i32>, expected: bool) {
        assert_eq!(expected, Solution1::can_jump(nums));
    }

    #[test_case(vec![0], true)]
    #[test_case(vec![2,3,1,1,4], true)]
    #[test_case(vec![3,2,1,0,4], false)]
    fn solve2(nums: Vec<i32>, expected: bool) {
        assert_eq!(expected, Solution2::can_jump(nums));
    }

    #[test_case(vec![0], true)]
    #[test_case(vec![2,3,1,1,4], true)]
    #[test_case(vec![3,2,1,0,4], false)]
    fn solve3(nums: Vec<i32>, expected: bool) {
        assert_eq!(expected, Solution3::can_jump(nums));
    }
}
