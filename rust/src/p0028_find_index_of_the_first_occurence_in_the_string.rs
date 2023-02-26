pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack.len() < needle.len() {
            return -1;
        }
        fn pre_kmp(needle: &[u8]) -> Vec<usize> {
            let mut kmp = vec![0; needle.len()];
            let mut pref = 0;
            for (suff, &c) in needle.iter().enumerate().skip(1) {
                while pref > 0 && needle[pref] != c {
                    pref = kmp[pref - 1];
                }
                if needle[pref] == c {
                    pref += 1;
                }
                kmp[suff] = pref;
            }
            kmp
        }
        let needle = needle.as_bytes();
        let kmp = pre_kmp(needle);
        let (mut needle_idx, last_needle_idx) = (0, needle.len() - 1);
        for (haystack_idx, c) in haystack.bytes().enumerate() {
            while needle_idx > 0 && needle[needle_idx] != c {
                needle_idx = kmp[needle_idx - 1];
            }
            if needle[needle_idx] == c {
                if needle_idx == last_needle_idx {
                    return (haystack_idx - needle_idx) as i32;
                }
                needle_idx += 1;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;

    #[test_case("onionionskys", "onions", 3)]
    #[test_case("sadbutsad", "sad", 0)]
    #[test_case("aaaaa", "bba", -1)]
    fn solve(haystack: &str, needle: &str, expected: i32) {
        assert_eq!(expected, Solution::str_str(String::from(haystack), String::from(needle)));
    }

}