pub struct Solution;

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        const MOD:i32 = 1_000_000_007;
        let (m, n) = (m as usize, n as usize);
        let (mut pdp, mut dp) = (vec![vec![0; n]; m], vec![vec![0; n]; m]);
        dp[start_row as usize][start_column as usize] = 1;
        let mut ans = 0;
        for cur_move in 1..=max_move {
            ans += dp[0].iter().copied().chain(dp[m-1].iter().copied()).chain(dp.iter().map(|row| (row[0] + row[n-1]) % MOD))
                .fold(0, |sum, el| (sum + el) % MOD);
            ans %= MOD;
            if cur_move==max_move {
                break;
            }
            std::mem::swap(&mut pdp, &mut dp);
            dp.iter_mut().for_each(|row| row.fill(0));
            for (row, dp_row) in dp.iter_mut().enumerate() {
                for (col, dp_el) in dp_row.iter_mut().enumerate() {
                    let moves = [
                        row.checked_sub(1).zip(Some(col)),
                        Some(row).zip(Some(col+1).filter(|&col| col<n)),
                        Some(row+1).filter(|&row| row<m).zip(Some(col)),
                        Some(row).zip(col.checked_sub(1))
                    ];
                    for (adj_row, adj_col) in moves.iter().filter_map(|&mv| mv) {
                        *dp_el += pdp[adj_row][adj_col];
                        *dp_el %= MOD;
                    }
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;

    #[test_case(2, 2, 2, 0, 0, 6)]
    #[test_case(1, 3, 3, 0, 1, 12)]
    fn solve(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32, expected: i32) {
        assert_eq!(expected, Solution::find_paths(m, n, max_move, start_row, start_column));
    }
}
