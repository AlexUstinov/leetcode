pub struct Solution;

impl Solution {
    pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
        let n = lcp.len();
        let mut bytes = vec![0; n];
        let mut c = b'a';
        for i in 0..n {
            if bytes[i]==0 {
                if c > b'z' {
                    return String::from("");
                }
                for (j, byte) in bytes[i..].iter_mut().enumerate() {
                    if lcp[i][i+j] > 0 && *byte == 0 {
                        *byte = c;
                    }
                }
                c += 1;
            }
        }
        for i in 0..n {
            for j in i..n {
                let v = lcp.get(i+1).and_then(|row| row.get(j+1)).unwrap_or(&0);
                let v = if bytes[i]==bytes[j] { v + 1 } else { 0 };
                if lcp[i][j] != lcp[j][i] || lcp[i][j]!=v {
                    return String::from("");                    
                }
            }
        }

        String::from_utf8(bytes).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;
    use crate::util::parse_matrix;

    #[test_case(parse_matrix("[[3,0,1],[0,2,1],[1,1,1]]"), "")]
    #[test_case(parse_matrix("[[8,0,0,0,0,1,2,0],[0,7,0,1,1,0,0,1],[0,0,6,0,0,0,0,0],[0,1,0,5,1,0,0,1],[0,1,0,1,4,0,0,1],[1,0,0,0,0,3,1,0],[2,0,0,0,0,1,2,0],[0,1,0,1,1,0,0,1]]"), "abcbbaab")]
    #[test_case(parse_matrix("[[4,0,2,0],[0,3,0,1],[2,0,2,0],[0,1,0,1]]"), "abab")]
    fn solve(lcp: Vec<Vec<i32>>, expected: &str) {
        assert_eq!(expected, Solution::find_the_string(lcp));
    }

}