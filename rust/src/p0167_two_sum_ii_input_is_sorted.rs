pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        std::iter::successors(Some((0, numbers.len() - 1)),
            |(l, r)| match numbers[*l] + numbers[*r] {
                t if t < target => Some((*l + 1, *r)),
                t if t > target => Some((*l, *r - 1)),
                _ => None,
            })
            .last()
            .map(|(l, r)| vec![l as i32 + 1, r as i32 + 1])
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;

    #[test_case(vec![2,7,11,15], 9, vec![1,2])]
    #[test_case(vec![2,3,4], 6, vec![1,3])]
    #[test_case(vec![-1,0], -1, vec![1,2])]
    fn solve(numbers: Vec<i32>, target: i32, expected: Vec<i32>) {
        assert_eq!(expected, Solution::two_sum(numbers, target));
    }
}