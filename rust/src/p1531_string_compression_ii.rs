pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        fn get_compressed_len(dp: &mut HashMap<(usize, i32, Option<(u8, i32)>), i32>, s: &[u8], k: i32, tail: Option<(u8, i32)>) -> i32 {
            fn get_encoded_len(len: i32) -> i32 { 
                i32::from(len>0) + i32::from(len>1) + i32::from(len>9) + i32::from(len>99)
            }
            if s.is_empty() {
                return tail.map_or(0,|(_, len)| get_encoded_len(len));
            }
            if let Some(&len) = dp.get(&(s.len(), k, tail)) {
                return len;
            }
            let new_tail_count = tail.and_then(|(tail_char, tail_count)| (s[0]==tail_char).then_some(tail_count+1)).unwrap_or(1);
            let d_len = tail.and_then(|(tail_char, tail_count)| (s[0]!=tail_char).then(|| get_encoded_len(tail_count))).unwrap_or(0);
            let len = [
                (k>0).then(|| get_compressed_len(dp, &s[1..], k-1, tail)),
                Some(get_compressed_len(dp, &s[1..], k, Some((s[0], new_tail_count))) + d_len)
            ];
            let len = len.into_iter().flatten().min().unwrap();
            dp.insert((s.len(), k, tail), len);
            len
        }
        get_compressed_len(&mut HashMap::new(), s.as_bytes(), k, None)
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