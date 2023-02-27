pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut start = 0;
        let mut mins = vec![];
        let mut maxes = vec![];
        let last = nums.len() - 1;
        let mut count = 0usize;
        for (i, val) in nums.iter().copied().enumerate() {
            match val {
                val if val < min_k || val > max_k || i == last => {
                    if min_k == max_k {
                        let end = if val==max_k { i + 1 } else { i };
                        let n = end - start;
                        count += (n * (n + 1)) / 2;
                    }
                    else {
                        if val==max_k {
                            maxes.push(i);
                        }
                        else if val==min_k {
                            mins.push(i);
                        }
                        let mut end = if val >= min_k && val <= max_k { i + 1 } else { i };
                        while let Some((min_idx, max_idx)) = mins.pop().zip(maxes.pop()) {
                            if min_idx < max_idx {
                                count += (end - max_idx) * (min_idx - start + 1);
                                mins.push(min_idx);
                                end = max_idx;
                            }
                            else {
                                count += (end - min_idx) * (max_idx - start + 1);
                                maxes.push(max_idx);
                                end = min_idx;
                            }
                        }
                    }

                    start = i + 1;
                    mins.clear();
                    maxes.clear();
                },
                val if val == min_k => mins.push(i),
                val if val == max_k => maxes.push(i),
                _ => {}
            }
        }
        count as i64
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;

    #[test_case(vec![1,5,2,5], 1, 5, 3)]
    #[test_case(vec![5,1,2,1], 1, 5, 3)]
    #[test_case(vec![5,2,5,1], 1, 5, 3)]
    #[test_case(vec![1,2,1,5], 1, 5, 3)]
    #[test_case(vec![7,5,1,5,7], 1, 7, 5)]
    #[test_case(vec![1,5,7,5,1], 1, 7, 5)]
    #[test_case(vec![1,1,1], 1, 1, 6)]
    #[test_case(vec![1,1,2,1,1], 1, 1, 6)]
    #[test_case(vec![2,1,1,2,1,1,2], 1, 1, 6)]
    #[test_case(vec![1,3,5,2,7,5], 1, 5, 2)]
    #[test_case(vec![1,3,5,7,1,2,5,1], 1, 5, 5)]
    #[test_case(vec![1,2,5,1,4], 1, 5, 7)]
    fn solve(nums: Vec<i32>, min_k: i32, max_k: i32, expected: i64) {
        assert_eq!(expected, Solution::count_subarrays(nums, min_k, max_k));
    }
}