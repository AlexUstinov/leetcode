pub struct Solution;

impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut distances = vec![vec![0; n]; n];
        let mut land_count = 0;
        for i in 0..n {
            for j in 0..n {
                distances[i][j] = if grid[i][j]==1 {
                    land_count += 1;
                    0
                }
                else {
                    let hrzl_dist = j.checked_sub(1).map_or(i32::MAX, |prev_j| distances[i][prev_j]);
                    let vrtl_dist = i.checked_sub(1).map_or(i32::MAX, |prev_i| distances[prev_i][j]);
                    hrzl_dist.min(vrtl_dist).saturating_add(1)
                }
            }
        }
        if land_count == 0 || land_count == n * n {
            return -1;
        }

        let mut max_distance = 0;
        for i in (0..n).rev() {
            for j in (0..n).rev() {
                let distance = if grid[i][j]==1 {
                    0
                } else {
                    let hrzl_dist = distances[i].get(j + 1).copied().unwrap_or(i32::MAX);
                    let vrtl_dist = distances.get(i + 1).map_or(i32::MAX, |row| row[j]);
                    hrzl_dist.min(vrtl_dist).saturating_add(1)
                };
                distances[i][j] = distance.min(distances[i][j]);
                max_distance = max_distance.max(distances[i][j]);
            }
        }

        max_distance
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;
    use crate::util::parse_matrix;

    #[test_case(parse_matrix("[[0,0,1,1,1],[0,1,1,0,0],[0,0,1,1,0],[1,0,0,0,0],[1,1,0,0,1]]"), 2)]
    #[test_case(parse_matrix("[[1,0,0,1],[0,0,0,0],[0,0,0,0],[1,0,0,1]"), 2)]
    fn solve (grid: Vec<Vec<i32>>, expected: i32) {
        assert_eq!(expected, Solution::max_distance(grid));
    }

}