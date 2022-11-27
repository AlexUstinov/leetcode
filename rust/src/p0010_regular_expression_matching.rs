pub struct TopDownSolution;
pub struct BottomUpSolution;

impl TopDownSolution {
    pub fn is_match(s: String, p: String) -> bool {
        let string_size = s.len();
        let pattern_size = p.len();
        let mut dp: Vec<Option<bool>> = vec![None; (string_size + 1) * (pattern_size + 1)];

        fn is_match_dp(dp: &mut Vec<Option<bool>>, text: &[u8], pattern: &[u8], row_size: usize) -> bool {
            match (text.len(), pattern.len()) {
                (0, 0) => true,
                (_, 0) => false,
                (text_size, pattern_size) => {
                    match pattern_size {
                        1 => text_size==1 && (pattern[0] == b'.' || pattern[0] == text[0]),
                        _ => match dp[text_size * row_size + pattern_size] {
                            Some(cached_match) => cached_match, 
                            None => {
                                let is_match = match pattern[1] {
                                    b'*' => is_match_dp(dp, text, &pattern[2..], row_size)
                                        || (text_size > 0 && ((pattern[0] == b'.' || pattern[0] == text[0]) && is_match_dp(dp, &text[1..], pattern, row_size))),
                                    _ => text_size > 0 && (pattern[0] == b'.' || pattern[0] == text[0]) && is_match_dp(dp, &text[1..], &pattern[1..], row_size)
                                };
                                dp[text_size * row_size + pattern_size] = Some(is_match);
                                is_match
                            }
                        }
                    }
                }

            }
        }

        is_match_dp(&mut dp, s.as_bytes(), p.as_bytes(), pattern_size + 1)
    }
}

impl BottomUpSolution {
    pub fn is_match(s: String, p: String) -> bool {
        let (m, n) = (s.len(), p.len());
        let mut dp = vec![false; (m + 1) * (n + 1)];
        dp[0] = true;
        let s_bytes = s.as_bytes();
        let p_bytes = p.as_bytes();
        if n > 1 {
            for i in (1..n).step_by(2) {
                dp[i + 1] = p_bytes[i] == b'*' && dp[i - 1];
            }
        }

        for i in 0..m {
            for j in 0..n {
                dp[(i + 1) * (n + 1) + j + 1] = match p_bytes[j] {
                    b'*' => {
                        let c = p_bytes[j - 1];
                        dp[(i + 1) * (n + 1) + j - 1] || (dp[i * (n + 1) + j + 1] && (c == b'.' || c == s_bytes[i]))
                    }
                    c => dp[i * (n + 1) + j] && (c == b'.' || c == s_bytes[i]),
                }
            }
        }

        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;


    #[test_case("aa", "a", false)]
    #[test_case("cc", "c*", true)]
    #[test_case("ab", ".*", true)]
    #[test_case("ab", ".*c", false)]
    #[test_case("abnormal", "ab.*a.", true)]
    fn solve(s: &str, p: &str, expected: bool) {
        assert_eq!(
            expected,
            TopDownSolution::is_match(String::from(s), String::from(p))
        );
        assert_eq!(
            expected,
            BottomUpSolution::is_match(String::from(s), String::from(p))
        );
    }
}
