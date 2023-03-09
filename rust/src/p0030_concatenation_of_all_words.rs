pub struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let w_cnt = words.len();
        let w_len = words[0].len();
        let mut ans = vec![];
        if s.len() < w_len * w_cnt {
            return ans;
        }
        let s = s.as_bytes();
        let words = words.iter().fold(std::collections::HashMap::new(), |mut map, w| { *map.entry(w.as_bytes()).or_insert(0) += 1; map });
        for start in 0..=s.len()-(w_len * w_cnt) {
            let mut s = &s[start..];
            let mut match_cnt = 0;
            let mut used_words = std::collections::HashMap::new();
            while match_cnt < w_cnt && s.len() >= w_len && {
                let w = &s[0..w_len];
                words.get(w).map_or(false, |cnt| cnt > used_words.get(w).unwrap_or(&0))
            } {
                *used_words.entry(&s[0..w_len]).or_insert(0) += 1;
                s = &s[w_len..];
                match_cnt += 1;
            }
            if match_cnt==w_cnt {
                ans.push(start as i32);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;

    #[test_case("a", vec!["a","a"], vec![])]
    #[test_case("wordgoodgoodgoodbestword", vec!["word","good","best","good"], vec![8])]
    fn solve(s: &str, words: Vec<&str>, expected: Vec<i32>) {
        assert_eq!(expected, Solution::find_substring(String::from(s), words.iter().copied().map(String::from).collect::<Vec<_>>()));
    }
}