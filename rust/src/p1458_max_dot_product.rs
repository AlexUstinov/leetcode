pub struct Solution;

impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (nums1, nums2) = if nums1.len() > nums2.len() { (nums2, nums1) } else { ( nums1, nums2 ) };
        let mut dp = vec![0; nums1.len()+1];
        let mut max_non_positive = i32::MIN;
        for num2 in nums2 {
            let mut prev = 0;
            for (i, num1) in nums1.iter().copied().enumerate() {
                let product = num1*num2;
                max_non_positive = product.max(max_non_positive);
                let max = (prev + product).max(dp[i]).max(dp[i+1]);
                prev = dp[i+1];
                dp[i+1] = max;
            }
        }
        let ans = dp[nums1.len()];
        if ans==0 { max_non_positive } else { ans }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;
    
    #[test_case(vec![2,1,-2,5], vec![3,0,-6], 18)]
    #[test_case(vec![3,-2], vec![2,-6,7], 21)]
    #[test_case(vec![-1,-1], vec![1, 1], -1)]
    #[test_case(vec![-3,-8,3,-10,1,3,9], vec![9,2,3,7,-9,1,-8,5,-1,-1], 200)]
    fn solve(nums1: Vec<i32>, nums2: Vec<i32>, expected: i32) {
        assert_eq!(expected, Solution::max_dot_product(nums1, nums2));
    }
}