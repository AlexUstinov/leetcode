pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let (m, n) = (haystack.len(), needle.len());
        if m < n {
            return -1;
        }
        fn pre_kmp(needle: &[u8]) -> Vec<usize> {
            let n = needle.len();
            let mut lcp = vec![0; n];
            let mut prev = 0;
            for i in 1..n {
                let a = needle[i];
                let mut b = needle[prev];
                while prev > 0 && a != b {
                    prev = lcp[prev - 1];
                    b = needle[prev];
                }
                if a == b {
                    prev += 1;
                }
                lcp[i] = prev
            }

            lcp
        }
        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();
        let lcp = pre_kmp(needle);
        let mut j = 0;
        for (i, &a) in haystack.iter().enumerate() {
            let mut b = needle[j];
            while j > 0 && a != b {
                j = lcp[j - 1];
                b = needle[j];
            }
            if a == b {
                j += 1;
                if j==n {
                    return (i + 1 - n) as i32;
                }
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
    fn solve(haystack: &str, needle: &str, expected: i32) {
        assert_eq!(expected, Solution::str_str(String::from(haystack), String::from(needle)));
    }

}