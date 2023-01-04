pub struct Solution;

impl Solution {
    pub fn minimum_rounds(mut tasks: Vec<i32>) -> i32 {
        use std::iter;
        tasks.sort_unstable();
        let mut count = 0;
        let (mut task_cnt, mut current) = (0, tasks[0] - 1);
        for task in tasks.iter().copied().chain(iter::once(tasks[tasks.len()-1] + 1)) {
            if current==task {
                task_cnt += 1;
                continue;
            }
            if task_cnt==1 {
                return -1;
            }
            let mut three_rnd_cnt = task_cnt / 3;
            if (task_cnt % 3) % 2 !=0 {
                three_rnd_cnt -= 1;
            }
            count += three_rnd_cnt + (task_cnt - three_rnd_cnt*3) / 2;
            task_cnt = 1;
            current = task;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;

    #[test_case(vec![2,2,3,3,2,4,4,4,4,4], 4)]
    #[test_case(vec![2,3,3], -1)]
    #[test_case(vec![2,2,3], -1)]
    fn solve(tasks: Vec<i32>, expected: i32) {
        assert_eq!(expected, Solution::minimum_rounds(tasks));
    }
}