use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_indexes = HashMap::new();
        for (sys_index, num) in nums.iter().enumerate() {
            let index = sys_index as i32;
            let compliment = target - num;
            if let Some(result) = num_indexes.get(&compliment) {
                return vec![*result, index];
            }
            num_indexes.insert(num, index);
        }
        panic!("No solution found"); 
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;
    use super::*;

    #[test_case(vec![2, 7, 11, 15], 9, vec![0, 1])]
    #[test_case(vec![3, 2, 4], 6, vec![1, 2])]
    #[test_case(vec![3, 3], 6, vec![0, 1])]
    fn solve(nums: Vec<i32>, target: i32, expected: Vec<i32>) {
        assert_eq!(Solution::two_sum(nums, target), expected)
    }
}