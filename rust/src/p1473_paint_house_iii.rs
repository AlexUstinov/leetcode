pub struct Solution;

impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        let t = target as usize;
        let (_m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![None; t+1]; n];
        let mut dp_next = dp.clone();
        for (color, dp_row) in dp.iter_mut().enumerate() {
            if color+1 != houses[0] as usize {
                dp_row[0] = Some(0);
            }
        }
        for (i, house) in houses.into_iter().enumerate() {
            for row in dp_next.iter_mut() {
                row.fill(None);
            }
            let can_paint = house==0;
            let house_colors = if can_paint { 0..n } else { let clr = (house - 1) as usize; clr..(clr+1) } ;
            for house_color in house_colors {
                let paint_costs = &mut dp_next[house_color];
                let house_cost = if can_paint { cost[i][house_color] } else { 0 };
                for prev_house_color in 0..n {
                    let prev_paint_costs = &dp[prev_house_color];
                    for num_neighborhoods in 0..=(t.min(i)) {
                        if let Some(prev_cost) = prev_paint_costs[num_neighborhoods] {
                            if house_color==prev_house_color {
                                paint_costs[num_neighborhoods] = Some(prev_cost + house_cost);
                            } else if let Some(paint_cost) = paint_costs.get_mut(num_neighborhoods+1) {
                                let candidate_cost:i32 = prev_cost + house_cost;
                                *paint_cost = Some(candidate_cost.min(paint_cost.as_ref().copied().unwrap_or(i32::MAX)));
                            }
                        }
                    }
                }
            }
            std::mem::swap(&mut dp, &mut dp_next);
        }
        dp.iter().filter_map(|paint_costs| paint_costs[t]).min().unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;
    use crate::util::parse_matrix;

    #[test_case(vec![1,2], parse_matrix("[[1,1],[1,1]]"), 2, 2, 1, -1)]
    fn solve(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32, expected: i32) {
        assert_eq!(expected, Solution::min_cost(houses, cost, m, n, target));
    }
}