pub struct Solution;

impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        const MOD:i32 = 1_000_000_007;
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;
        let mut cdp = dp.clone();
        for d in 1..=n {
            for kk in 1..=k {
                let min = (d-1+kk).min(target+1);
                let max = ((d-1)*k+kk).min(target);
                let range = min..=max;
                for sum in range {
                    if sum >= kk {
                        cdp[sum as usize] += dp[(sum-kk) as usize];
                        cdp[sum as usize] %= MOD;
                    }
                }
            }
            std::mem::swap(&mut dp, &mut cdp);
            cdp.fill(0);
        }

        dp[target as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use test_case::test_case;

    #[test_case(1, 6, 6, 1)]
    #[test_case(2, 12, 8, 7)]
    fn solve(n:i32, k:i32, target:i32, expected:i32) {
        assert_eq!(expected, Solution::num_rolls_to_target(n, k, target));
    }
}