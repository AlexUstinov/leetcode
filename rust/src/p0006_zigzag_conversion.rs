pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut result = String::from("");
        for byte in s.zigzag_iter(num_rows) {
            result.push(char::from(byte));
        }
        result
    }
}

trait ZigZagIterable {
    type ZZIter;

    fn zigzag_iter(self, num_rows: i32) -> Self::ZZIter;
}

impl<'a> ZigZagIterable for &'a str {
    type ZZIter = ZigZagIterator<'a>;
    fn zigzag_iter(self, num_rows: i32) -> Self::ZZIter {
        ZigZagIterator {
            s: self,
            num_rows,
            row: 0
        }
    }    
}

struct ZigZagIterator<'a> {
    s: &'a str,
    num_rows: i32,
    row: i32
}

impl<'a> Iterator for ZigZagIterator<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;
    use super::Solution;

    #[test_case(String::from("PAYPALISHIRING"), 3, "PAHNAPLSIIGYIR")]
    #[test_case(String::from("PAYPALISHIRING"), 4, "PINALSIGYAHRPI")]
    fn solve(s: String, num_rows: i32, expected: &str) {
        assert_eq!(expected, Solution::convert(s, num_rows));
    }
}