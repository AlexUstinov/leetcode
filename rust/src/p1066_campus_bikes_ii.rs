pub struct Solution;

impl Solution {
    pub fn assign_bikes(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::util::parse_pairs;
    use super::Solution;

    #[test_case("[[0,0],[2,1]]", "[[1,2],[3,3]]", 6)]
    #[test_case("[[0,0],[1,1],[2,0]]", "[[1,0],[2,2],[2,1]]", 4)]
    fn solve(workers: &str, bikes: &str, expected: i32) {
        assert_eq!(expected, Solution::assign_bikes(parse_pairs(workers), parse_pairs(bikes)));
    }

}