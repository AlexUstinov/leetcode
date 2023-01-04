pub struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut lkp = HashMap::new();
        let mut words = HashSet::new();
        let (mut char_iter, mut word_iter) = (pattern.chars(), s.split(' '));
        for (c, word) in &mut char_iter.zip(&mut word_iter) {
            match lkp.get(&c) {
                Some(prev_word) if prev_word!=&word => { return false; },
                None => {
                    if words.get(word).is_some() {
                        return false;
                    }
                    lkp.insert(c, word);
                    words.insert(word)
                },
                _ => continue
            };
        }
        char_iter.next().is_none() && word_iter.next().is_none()
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