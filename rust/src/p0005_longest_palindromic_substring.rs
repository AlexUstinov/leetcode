struct Solution;

impl Solution {
    fn expand(s_bytes: &[u8], mut l: usize, mut r: usize) -> Option<String> {
        if r >= s_bytes.len() || s_bytes[l]!=s_bytes[r] {
            return None;
        }
        loop {
            if let (Some(new_l), Some(new_r)) = (l.checked_sub(1), r.checked_add(1)) {
                if new_r < s_bytes.len() && s_bytes[new_l]==s_bytes[new_r] {
                    (l, r) = (new_l, new_r);
                    continue;
                }
            }
            break match String::from_utf8(s_bytes[l..(r+1)].to_vec()) {
                Ok(result) => Some(result),
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e)
            }
        }
    }

    pub fn longest_palindrome(s: String) -> String {
        let s_bytes = s.as_bytes();
        let mut center = 0usize;
        let mut max_palindrome = String::from("");
        while center < s_bytes.len() {
            if let Some(max_candidate) = Solution::expand(s_bytes, center, center) {
                if max_palindrome.len() < max_candidate.len() {
                    max_palindrome = max_candidate;
                }
            }
            if let Some(max_candidate) = Solution::expand(s_bytes, center, center+1) {
                if max_palindrome.len() < max_candidate.len() {
                    max_palindrome = max_candidate;
                }
            }
            center += 1;
        }
        max_palindrome
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;
    use super::Solution;

    #[test_case(String::from("abcabcbb"), "bcb")]
    fn solve(s: String, expected: &str) {
        assert_eq!(&Solution::longest_palindrome(s), expected);
    }
}