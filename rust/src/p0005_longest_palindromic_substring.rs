struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        fn expand(s_bytes: &[u8], mut l: usize, mut r: usize) -> Option<&[u8]> {
            if r >= s_bytes.len() || s_bytes[l] != s_bytes[r] {
                return None;
            }
            loop {
                if let (Some(new_l), Some(new_r)) = (l.checked_sub(1), r.checked_add(1)) {
                    if new_r < s_bytes.len() && s_bytes[new_l] == s_bytes[new_r] {
                        (l, r) = (new_l, new_r);
                        continue;
                    }
                }
                break Some(&s_bytes[l..(r + 1)]);
            }
        }

        let s_bytes = s.as_bytes();
        let mut center = 0usize;
        let mut max_palindrome: &[u8] = &[];
        while center < s_bytes.len() {
            if let Some(max_candidate) = expand(s_bytes, center, center) {
                if max_palindrome.len() < max_candidate.len() {
                    max_palindrome = max_candidate;
                }
            }
            if let Some(max_candidate) = expand(s_bytes, center, center + 1) {
                if max_palindrome.len() < max_candidate.len() {
                    max_palindrome = max_candidate;
                }
            }
            center += 1;
        }
        match std::str::from_utf8(max_palindrome) {
            Ok(result) => String::from(result),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use test_case::test_case;

    #[test_case(String::from("abcabcbb"), "bcb")]
    fn solve(s: String, expected: &str) {
        assert_eq!(&Solution::longest_palindrome(s), expected);
    }
}
