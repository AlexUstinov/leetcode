pub struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut lkp = HashMap::new();
        let mut words = HashSet::new();
        let word_vec = s.split(' ').collect::<Vec<_>>();
        if pattern.len() != word_vec.len() {
            return false;
        }
        for (c, word) in pattern.chars().zip(word_vec.iter()) {
            match lkp.get(&c) {
                Some(prev_word) if prev_word!=&word => { return false; },
                None => {
                    if words.contains(word) {
                        return false;
                    }
                    lkp.insert(c, word);
                    words.insert(word)
                },
                _ => continue
            };
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;

    #[test_case(String::from("abba"), String::from("dog cat cat dog"), true)]
    #[test_case(String::from("abba"), String::from("dog cat cat fish"), false)]
    #[test_case(String::from("abba"), String::from("dog dog dog dog"), false)]
    #[test_case(String::from("aaa"), String::from("aa aa aa aa"), false)]
    fn solve(pattern: String, s: String, expected: bool) {
        assert_eq!(expected, Solution::word_pattern(pattern, s));
    }

}