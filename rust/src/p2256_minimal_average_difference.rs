pub struct Solution;

impl Solution {
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i64;
        if n==1 {
            return 0;
        }
        let (mut left_sum, mut right_sum) = (nums[0] as i64, nums.iter().skip(1).fold(0i64, |sum, num| sum + (*num as i64)));
        let (mut left_count, mut right_count) = (1, n - 1);
        let (mut min_diff, mut min_diff_idx) = ((right_sum / right_count - left_sum / left_count).abs(), 0);
        for (idx, num) in nums.iter().enumerate().skip(1) {
            left_sum += *num as i64;
            left_count += 1;
            right_sum -= *num as i64;
            right_count -= 1;
            let diff = if right_count==0 { (left_sum / left_count).abs() } else { (right_sum / right_count - left_sum / left_count).abs() };
            if diff < min_diff {
                min_diff = diff;
                min_diff_idx = idx as i32;
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