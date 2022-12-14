use std::mem;

pub struct Solution;

impl Solution {
    pub fn longest_common_subsequence(mut text1: String, mut text2: String) -> i32 {
        if text1.len() > text2.len() {
            mem::swap(&mut text1, &mut text2);
        }
        let text1_len = text1.len();
        let (mut dp_prev, mut dp) = (vec![0; text1_len], vec![0; text1_len]);
        for c2 in text2.chars() {
            mem::swap(&mut dp_prev, &mut dp);
            for (i, c1) in text1.chars().enumerate() {
                dp[i] = match i.checked_sub(1) {
                    Some(i_1) if c1==c2 => dp_prev[i_1] + 1,
                    Some(i_1) => dp_prev[i].max(dp[i_1]),
                    None if c1==c2 => 1,
                    None => dp_prev[i]
                }
            }
        }
        dp[text1_len - 1]
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;

    #[test_case("abcde", "ace", 3)]
    #[test_case("abc", "abc", 3)]
    #[test_case("abc", "def", 0)]
    fn solve(text1: &str, text2: &str, expected: i32) {
        assert_eq!(expected, Solution::longest_common_subsequence(String::from(text1), String::from(text2)));
    }
}