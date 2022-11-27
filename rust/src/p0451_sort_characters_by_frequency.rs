pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut frequencies = HashMap::<u8, i32>::new();
        let mut bytes = s.as_bytes().to_vec();
        for c in &bytes {
            match frequencies.get(c) {
                Some(f) => frequencies.insert(*c, f + 1),
                None => frequencies.insert(*c, 1),
            };
        }
        bytes.sort_by(|a, b| {
            let x = (frequencies.get(b).unwrap(), b);
            let y = (frequencies.get(a).unwrap(), a);
            x.cmp(&y)
        } );
        String::from_utf8(bytes).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test_case::test_case(String::from("abbccc"), "cccbba")]
    fn solve(s: String, expected: &str) {
        assert_eq!(expected, Solution::frequency_sort(s));
    }

}

