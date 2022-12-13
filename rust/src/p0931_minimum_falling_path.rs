pub struct Solution;

use std::cmp;
impl Solution {
    pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
        for row_idx in 1..matrix.len() {
            let row_len = matrix[row_idx].len();
            for el_idx in 0..row_len {
                let prev_row = &matrix[row_idx - 1];
                let mut min_path = cmp::min(prev_row[el_idx], prev_row[el_idx.saturating_sub(1)]);
                if el_idx < row_len - 1 {
                    min_path = cmp::min(min_path, prev_row[el_idx + 1]);
                }
                matrix[row_idx][el_idx] += min_path;
            }
        }
        *matrix.last().expect("Matrix must not be empty.")
            .iter().min().expect("Row must not be empty.")
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;

    #[test_case("[[2,1,3],[6,5,4],[7,8,9]]", 13)]
    #[test_case("[[-19,57],[-40,-5]]", -59)]
    #[test_case("[[1]]", 1)]
    fn solve(matrix: &str, expected: i32) {
        assert_eq!(expected, Solution::min_falling_path_sum(self::parse_matrix(matrix)));
    }

    fn parse_matrix(matrix: &str) -> Vec<Vec<i32>> {
        let trim_pat: &[_] = &[' ', '[', ']'];
        let rows = matrix
            .trim_matches(trim_pat)
            .split("],[");
        let mut result = Vec::<Vec<i32>>::new();
        for row in rows {
            result.push(row.trim_matches(trim_pat).split(',').map(|el| el.parse().unwrap()).collect());
        }
        result
    }
}