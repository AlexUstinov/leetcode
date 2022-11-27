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
            next_el: None
        }
    }    
}

struct ZigZagIterator<'a> {
    bytes: &'a [u8],
    num_rows: i32,
    row: i32,
    idx: usize,
    next_el: Option<u8>
}

impl<'a> Iterator for ZigZagIterator<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        match self.next_el {
            Some(el) => {
                self.next_el = None;
                Some(el)
            },
            None => {
                let len = self.bytes.len();
                if self.idx >= len {
                    self.row += 1;
                    self.idx = 0;
                    if self.row == self.num_rows {
                        return None;
                    }
                }
                if self.row==0 {
                    let el = self.bytes[self.idx];
                    self.idx += (2*self.num_rows - 2) as usize;
                    Some(el)
                }
                else if self.row == self.num_rows - 1 {
                    let el_idx = self.idx + (self.row as usize);
                    let el = if el_idx < len { Some(self.bytes[el_idx]) } else { None };
                    self.idx += (2*self.num_rows - 2) as usize;
                    el
                }
                else {
                    let idx1 = self.idx.checked_sub(self.row as usize);
                    let idx2 = self.idx + (self.row as usize);
                    self.idx += (2*self.num_rows - 2) as usize;
                    match idx1 {
                        Some(idx1) => {
                            if idx2 < len {
                                self.next_el = Some(self.bytes[idx2]);
                            }
                            Some(self.bytes[idx1])
                        },
                        None => {
                            Some(self.bytes[idx2])
                        }
                    }
                }
            }
        }
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