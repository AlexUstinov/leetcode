pub struct Solution;

use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
struct DpKey(usize, i32, Option<(u8, i32)>);

impl Solution {
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        fn get_compressed_len(dp: &mut HashMap<DpKey, i32>, s: &[u8], k: i32, tail: Option<(u8, i32)>) -> Option<i32> {
            fn get_encoded_len(len: i32) -> i32 { 
                i32::from(len>0) + i32::from(len>1) + i32::from(len>9) + i32::from(len>99)
            }
            if k < 0 {
                return None;
            }
            if s.is_empty() {
                return tail.map(|(_, len)| get_encoded_len(len));
            }
            let cached = dp.get(&DpKey(s.len(), k, tail)).copied();
            if cached.is_some() {
                return cached;
            }
            let (tail_len, d_len) = tail.map_or((1, 0), |(tail_char, tail_len)| if s[0]==tail_char { (tail_len+1, 0) } else { (1, get_encoded_len(tail_len)) });
            let len = [
                get_compressed_len(dp, &s[1..], k-1, tail),
                get_compressed_len(dp, &s[1..], k, Some((s[0], tail_len))).map(|len| len + d_len)
            ];
            let len = len.into_iter().flatten().min().unwrap();
            dp.insert(DpKey(s.len(), k, tail), len);
            Some(len)
        }
        get_compressed_len(&mut HashMap::new(), s.as_bytes(), k, None).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use test_case::test_case;

    #[test_case(String::from("abc"), 2, 1)]
    #[test_case(String::from("aaabcccd"), 2, 4)]
    #[test_case(String::from("abaa"), 1, 2)]
    fn solve(s:String, k:i32, expected:i32) {
        assert_eq!(expected, Solution::get_length_of_optimal_compression(s, k));
    }
}