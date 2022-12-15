pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut indexed_temps: Vec<(i32, usize)> = temperatures.iter().enumerate().map(|(idx, t)| (*t, idx)).collect();
        indexed_temps.sort();
        let len = temperatures.len();
        let mut result = vec![0; len];
        for (idx, t) in temperatures.iter().enumerate() {
            let mut t = *t;
            let mut next_warm = i32::MAX;
            result[idx] = loop {
                t += 1;
                if t >= 100 {
                    break if next_warm==i32::MAX { 0 } else {next_warm};
                }
                let key = (t, idx);
                match indexed_temps.binary_search(&key) {
                    Err(w_idx) if w_idx < len && indexed_temps[w_idx].1 > idx => next_warm = next_warm.min((indexed_temps[w_idx].1 - idx) as i32),
                    _ => continue
                };    
            };
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;

    #[test_case(vec![73,74,75,71,69,72,76,73], vec![1,1,4,2,1,1,0,0])]
    fn solve(temperatures: Vec<i32>, expected: Vec<i32>) {
        assert_eq!(expected, Solution::daily_temperatures(temperatures));
    }
}