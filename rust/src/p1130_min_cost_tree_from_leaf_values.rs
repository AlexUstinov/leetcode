pub struct Solution;

impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut dp = (0..n).fold(Vec::<Vec<(i32, i32)>>::with_capacity(n), |mut dp, idx| { dp.push(vec![(0, 0); n-idx]); dp });

        for rank in 0..n {
            for row in 0..(n-rank) {
                dp[row][rank] = match rank {
                    0 => (0, arr[row]),
                    _ => (0..rank).fold((i32::MAX, 0), |(sum, leaf_val), k| {
                        let (l_sum, l_leaf_val) = dp[row][k];
                        let (r_sum, r_leaf_val) = dp[row+k+1][rank-k-1];
                        let val = l_leaf_val * r_leaf_val;

                        (sum.min(l_sum+r_sum+val), leaf_val.max(l_leaf_val).max(r_leaf_val))
                    })
                }
            }
        }

        dp[0][n-1].0
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;

    #[test_case(vec![4,11], 44)]
    #[test_case(vec![6,2,4], 32)]
    fn solve(arr: Vec<i32>, expected: i32) {
        assert_eq!(expected, Solution::mct_from_leaf_values(arr));
    }
}