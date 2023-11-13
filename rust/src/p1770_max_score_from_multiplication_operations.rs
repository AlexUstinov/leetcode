pub struct Solution {}
// Let factor = f(k), num = N(i)
// Let denote length of remained nums on k-th iteration as nums_len(k)
// score(i,k) = max(
//     f(k)*N(i) + score(i+1, k+1),
//     f(k)*N(i+nums_len(k)-1) + score(i, k+1)
// )
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let (n, m) = (nums.len(), multipliers.len());
        let mut score = vec![0; n+1];
        for k in (0..m).rev() {
            let nums_len = n - k;
            for i in 0..=n-nums_len {
                let factor = multipliers[k];
                score[i] = std::cmp::max(
                    factor*nums[i] + score[i+1],
                    factor*nums[i+nums_len-1] + score[i]
                );
            }
        }

        score[0]
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    #[test_case(vec![1,2,3], vec![3,2,1], 14)]
    #[test_case(vec![-5,-3,-3,-2,7,1], vec![-10,-5,3,4,6], 102)]
    fn solve(nums: Vec<i32>, multipliers: Vec<i32>, expected: i32) {
        assert_eq!(expected, Solution::maximum_score(nums, multipliers));
    }

}