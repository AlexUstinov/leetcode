pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let mut max_len = 0;
        let nums: HashSet<i32> = HashSet::from_iter(arr.iter().copied());
        for (i, b) in arr.iter().copied().enumerate().skip(1) {
            for a in arr[0..i].iter().copied() {
                let (mut a, mut b, mut len) = (a, b, 2);
                while let Some(c) = nums.get(&(a + b)).copied() {
                    len += 1;
                    max_len = max_len.max(len);
                    (a, b) = (b, c);
                }
            }
        }

        max_len
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3,4,5,6,7,8], 5)]
    #[test_case(vec![1,3,7,11,12,14,18], 3)]
    #[test_case(vec![2,5,6,7,8,10,12,17,24,41,65], 5)]
    fn solve(arr: Vec<i32>, expected: i32) {
        assert_eq!(expected, Solution::len_longest_fib_subseq(arr));
    }
}