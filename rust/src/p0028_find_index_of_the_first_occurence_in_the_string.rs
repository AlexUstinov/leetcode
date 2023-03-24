pub struct Solution1;
pub struct Solution2;

impl Solution1 {
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

impl Solution2 {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let needle_len = needle.len();
        if needle_len <= haystack.len() {
            let max_weight = (0..needle_len).fold(1, |weight, _| (weight * 26) % MOD);
            let needle_bytes = needle.as_bytes();
            let needle_hash = needle_bytes.iter().copied().fold(0, |hash, c| (hash*26 + (c - b'a') as i64) % MOD);
            let hay_bytes = haystack.as_bytes();
            let mut hay_hash = 0;
            let mut prev_start = None;
            for (i, w) in hay_bytes.windows(needle_len).enumerate() {
                hay_hash = match prev_start {
                    None => w.iter().copied().fold(0, |hash, c| (hash*26 + (c - b'a') as i64) % MOD),
                    Some(prev) => ((hay_hash * 26) % MOD - (prev * max_weight) % MOD + (w[needle_len - 1] - b'a') as i64 + MOD) % MOD
                };
                if hay_hash==needle_hash && w==needle_bytes {
                    return i as i32;
                }
                prev_start = Some((w[0] - b'a') as i64);
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    #[test_case("onionionskys", "onions", 3)]
    #[test_case("sadbutsad", "sad", 0)]
    #[test_case("aaaaa", "bba", -1)]
    fn solve1(haystack: &str, needle: &str, expected: i32) {
        assert_eq!(expected, Solution1::str_str(String::from(haystack), String::from(needle)));
    }
    #[test_case("onionionskys", "onions", 3)]
    #[test_case("sadbutsad", "sad", 0)]
    #[test_case("sadbutsad", "dbut", 2)]
    #[test_case("aaaaa", "bba", -1)]
    fn solve2(haystack: &str, needle: &str, expected: i32) {
        assert_eq!(expected, Solution2::str_str(String::from(haystack), String::from(needle)));
    }

}