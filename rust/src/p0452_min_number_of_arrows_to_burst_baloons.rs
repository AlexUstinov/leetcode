pub struct Solution;

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let mut pinch = i32::MAX;
        let mut result = 1;
        for itm in points.iter() {
            if itm[0] <= pinch {
                pinch = pinch.min(itm[1]);
                continue;
            }
            pinch = itm[1];
            result += 1
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::util::parse_pairs;
    use super::Solution;

    #[test_case(parse_pairs("[[10,16],[2,8],[1,6],[7,12]]"), 2)]
    #[test_case(parse_pairs("[[1,2],[3,4],[5,6],[7,8]]"), 4)]
    #[test_case(parse_pairs("[[1,2],[2,3],[3,4],[4,5]]"), 2)]
    fn solve(points: Vec<Vec<i32>>, expected: i32) {
        assert_eq!(expected, Solution::find_min_arrow_shots(points));
    }
}