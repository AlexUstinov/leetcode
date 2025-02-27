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
            bytes: self.as_bytes(),
            num_rows,
            row: -1,
            idx: usize::MAX,
            next_el: None,
        }
    }
}

struct ZigZagIterator<'a> {
    bytes: &'a [u8],
    num_rows: i32,
    row: i32,
    idx: usize,
    next_el: Option<u8>,
}

impl Iterator for ZigZagIterator<'_> {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        match self.next_el {
            Some(el) => {
                self.next_el = None;
                Some(el)
            }
            None => {
                let len = self.bytes.len();
                if self.row < 0 || self.idx >= len + (self.row as usize) {
                    self.row += 1;
                    self.idx = 0;
                    if self.row == self.num_rows {
                        return None;
                    }
                }
                let idx1 = self.idx.checked_sub(self.row as usize);
                let idx2 = self.idx + (self.row as usize);
                self.idx += if self.num_rows > 1 { (2 * self.num_rows - 2) as usize } else { 1 };
                match idx1 {
                    Some(idx1) if self.row < self.num_rows - 1 => {
                        self.next_el = if idx1 != idx2 && idx2 < len { Some(self.bytes[idx2]) } else { None };
                        Some(self.bytes[idx1])
                    }
                    _ => {
                        if idx2 < len { Some(self.bytes[idx2]) } else { None }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use test_case::test_case;

    #[test_case(String::from("PAYPALISHIRING"), 1, "PAYPALISHIRING")]
    #[test_case(String::from("PAYPALISHIRING"), 2, "PYAIHRNAPLSIIG")]
    #[test_case(String::from("PAYPALISHIRING"), 3, "PAHNAPLSIIGYIR")]
    #[test_case(String::from("PAYPALISHIRING"), 4, "PINALSIGYAHRPI")]
    #[test_case(String::from("PAYPALISHIRING"), 13, "PAYPALISHIRIGN")]
    #[test_case(String::from("PAYPALISHIRING"), 14, "PAYPALISHIRING")]
    #[test_case(String::from("PAYPALISHIRING"), 1000, "PAYPALISHIRING")]
    fn solve(s: String, num_rows: i32, expected: &str) {
        assert_eq!(expected, Solution::convert(s, num_rows));
    }
}
