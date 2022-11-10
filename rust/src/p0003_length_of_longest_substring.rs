use std::cmp;
use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn length_of_longest_substring(s: String) -> i32 {
        let input = s.as_bytes();
        let mut chars = [0; 128];

        let mut left: usize = 0;
        let mut right: usize = 0;

        let mut res = 0;

        while right < input.len() {
            let r = input[right] as usize;
            chars[r] += 1;

            while chars[r] > 1 {
                let l = input[left] as usize;
                chars[l] -= 1;
                left += 1;
            }

            res = cmp::max(res, right - left + 1);

            right += 1;
        }
        res as i32
    }
}

#[allow(dead_code)]
struct Solution1;
impl Solution1 {
    #[allow(dead_code)]
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_length = 0;
        let mut length = 0;
        let mut start_index = 0;
        let mut char_indices = HashMap::new();
        for (index, c) in s.chars().enumerate() {
            length = match char_indices.insert(c, index) {
                None => length + 1,
                Some(prev_index) if prev_index < start_index => length + 1,
                Some(prev_index) => {
                    start_index = prev_index + 1;
                    max_length = cmp::max(max_length, length);
                    (index - prev_index) as i32
                }
            }
        }
        cmp::max(max_length, length)
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;
    use super::*;

    #[test_case(String::from("abcabcbb"), 3)]
    #[test_case(String::from("bbbbb"), 1)]
    #[test_case(String::from("pwwkew"), 3)]
    fn solve(s: String, answer: i32) {
        assert_eq!(answer, Solution::length_of_longest_substring(s.clone()));
        assert_eq!(answer, Solution1::length_of_longest_substring(s));
    }
}