pub struct Solution;

impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, num_days: i32) -> i32 {
        let num_days = num_days as usize;
        let num_jobs = job_difficulty.len();
        let mut dp = vec![vec![None; job_difficulty.len()+1]; num_days+1];
        // dp[m_days][n_jobs] = min difficulty if last n_jobs scheduled in m_days

        for day_dp in dp.iter_mut() {
            day_dp[0] = Some(0);
        }
        let mut sum = 0;
        for (difficulty, dp_val) in job_difficulty.iter().rev().zip(dp[0].iter_mut().skip(1)) {
            sum += difficulty;
            *dp_val = Some(sum);
        }

        for day in 1..=num_days {
            for remained_jobs in day..=num_jobs {
                let mut day_max = 0;
                let mut min_schedule_difficulty = i32::MAX;
                for next_remained_jobs in (0..remained_jobs).rev() {
                    let job_idx = num_jobs - next_remained_jobs - 1;
                    day_max = day_max.max(job_difficulty[job_idx]);
                    min_schedule_difficulty = min_schedule_difficulty.min(dp[day-1][next_remained_jobs].map_or(day_max, |d| d+day_max));
                }
                dp[day][remained_jobs] = Some(min_schedule_difficulty);
            }
        }

        dp[num_days][num_jobs].unwrap_or(-1)

        // fn calculate(dp: &mut Vec<Vec<Option<i32>>>, difficulties: &[i32], d: usize) -> Option<i32> {
        //     match (difficulties.len(), d) {
        //         (len, d) if len < d => None,
        //         (len, d) if len == d || d == 0 => Some(difficulties.iter().copied().sum()),
        //         (len, d) if dp[d-1][len-1].is_some() => dp[d-1][len-1],
        //         (len, d) => {
        //             let mut schedule_difficulty = i32::MAX;
        //             let mut day_difficulty = 0;
        //             for i in 1..=difficulties.len() {
        //                 if let Some(next_schedule_difficulty) = calculate(dp, &difficulties[i..], d-1) {
        //                     day_difficulty = day_difficulty.max(difficulties[i-1]);
        //                     schedule_difficulty = schedule_difficulty.min(day_difficulty + next_schedule_difficulty);
        //                 } else {
        //                     break;
        //                 }
        //             }
        //             dp[d-1][len-1] = Some(schedule_difficulty);
        //             dp[d-1][len-1]
        //         }
        //     }
        // }

        // let d = d as usize;
        // calculate(&mut vec![vec![None; job_difficulty.len()]; d], &job_difficulty[..], d).unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use test_case::test_case;

    #[test_case(vec![6,5,4,3,2,1], 2, 7)]
    fn solve(job_difficulty: Vec<i32>, num_days: i32, expected: i32) {
        assert_eq!(expected, Solution::min_difficulty(job_difficulty, num_days));
    }
}