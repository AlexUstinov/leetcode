pub struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn connect_sticks(sticks: Vec<i32>) -> i32 {
        fn to_min_heap(mut sticks: Vec<i32>) -> BinaryHeap<i32> {
            for el in sticks.iter_mut() {
                *el = -*el;
            }
            BinaryHeap::from(sticks)
        }
        let mut sticks = to_min_heap(sticks);
        let mut total_cost = 0;
        while let (Some(a), Some(b)) = (sticks.pop(), sticks.pop()) {
            let cost = a+b;
            total_cost += cost;
            sticks.push(cost);
        }
        -total_cost
    }    
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;

    #[test_case(vec![2,4,3], 14)]
    #[test_case(vec![1,8,3,5], 30)]
    fn solve(sticks: Vec<i32>, expected: i32) {
        assert_eq!(expected, Solution::connect_sticks(sticks));
    }
}