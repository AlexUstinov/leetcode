pub struct Solution;

impl Solution {
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut left_sum, mut right_sum) = (nums.iter().map(|&el| el as i64).sum::<i64>(), 0);
        let (mut left_count, mut right_count) = (n as i64, 0);
        let (mut min_diff, mut min_diff_idx) = ((left_sum / left_count).abs(), n as i32 - 1);
        for (idx, &num) in nums.iter().enumerate().rev().take(n - 1) {
            left_sum -= num as i64;
            left_count -= 1;
            right_sum += num as i64;
            right_count += 1;
            let diff = (right_sum / right_count - left_sum / left_count).abs();
            if diff <= min_diff {
                min_diff = diff;
                min_diff_idx = idx as i32 - 1;
            }
        }
        min_diff_idx
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;
    use super::Solution;

    #[test_case(vec![2,5,3,9,5,3], 3)]
    #[test_case(vec![4,2,0], 2)]
    fn solve(nums: Vec<i32>, expected: i32) {
        assert_eq!(expected, Solution::minimum_average_difference(nums));
    }
}