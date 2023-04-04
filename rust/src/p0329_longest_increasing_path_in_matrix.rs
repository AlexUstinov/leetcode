pub struct Solution;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        fn get_adj(i:usize, j: usize, m: usize, n: usize) -> impl Iterator<Item = (usize, usize)> {
            IntoIterator::into_iter([
                i.checked_sub(1).map(|ii| (ii, j)),
                if j+1 < n { Some((i, j+1)) } else { None },
                if i+1 < m { Some((i+1, j)) } else { None },
                j.checked_sub(1).map(|jj| (i, jj))
            ]).flatten()
        }

        let (m, n) = (matrix.len(), matrix[0].len());
        let mut roots = vec![];
        for i in 0..m {
            for j in 0..n {
                if get_adj(i, j, m, n).all(|(ii, jj)| matrix[ii][jj] >= matrix[i][j]) {
                    roots.push((i, j));
                }
            }
        }

        let mut len = 0;
        let mut queue = std::collections::VecDeque::from(roots);
        let mut lvl_size = queue.len();
        while let Some((i, j)) = queue.pop_front() {
            for (ii, jj) in get_adj(i, j, m, n) {
                if matrix[ii][jj] > matrix[i][j] {
                    queue.push_back((ii, jj));
                }
            }
            lvl_size -= 1;
            if lvl_size == 0 {
                len += 1;
                lvl_size = queue.len();
            }
        }

        len
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;
    use crate::util::parse_matrix;

    #[test_case(parse_matrix("[[3,4,5],[3,2,6],[2,2,1]]"), 4)]
    fn solve(matrix: Vec<Vec<i32>>, expected: i32) {
        assert_eq!(expected, Solution::longest_increasing_path(matrix));
    }
}