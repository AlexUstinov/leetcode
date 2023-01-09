pub struct Solution;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() < 3 {
            return points.len() as i32;
        }
        let mut max_count = 0;
        for a_idx in 0..points.len() {
            let a = &points[a_idx];
            for b_idx in (a_idx+1)..points.len() {
                let b = &points[b_idx];
                let (vx, vy) = (b[1]-a[1], a[0]-b[0]);
                let mut count = 0;
                for c in &points[(b_idx+1)..] {
                    let (px, py) = (b[0]-c[0], b[1]-c[1]);
                    if vx*px + vy*py == 0 {
                        count += 1;
                    }
                }
                max_count = count.max(max_count);
            }
        }
        max_count + 2
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::util::parse_pairs;
    use super::Solution;

    #[test_case(parse_pairs("[[1,1]"), 1)]
    #[test_case(parse_pairs("[[1,1],[2,2],[3,3]]"), 3)]
    #[test_case(parse_pairs("[[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]"), 4)]
    fn solve(points: Vec<Vec<i32>>, expected: i32) {
        assert_eq!(expected, Solution::max_points(points));
    }
}