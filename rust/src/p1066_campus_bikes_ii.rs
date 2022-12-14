pub struct BruteforceSolution;

impl BruteforceSolution {
    pub fn assign_bikes(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> i32 {
        fn distance(a: &[i32], b: &[i32]) -> i32 {
            (a[0] - b[0]).abs() + (a[1] - b[1]).abs()
        }
        fn assign(assignment: i32, workers: &Vec<Vec<i32>>, bikes: &Vec<Vec<i32>>) -> i32 {
            let worker = assignment.count_ones() as usize;
            if worker < workers.len() {
                let mut min_assignment = i32::MAX;
                for (bike, bike_pos) in bikes.iter().enumerate() {
                    let bike_mask = 1 << bike;
                    if assignment & bike_mask == 0 {
                        let dist_to_bike = distance(&workers[worker], bike_pos);
                        let total_distance = dist_to_bike + assign(assignment | bike_mask, workers, bikes);
                        min_assignment = min_assignment.min(total_distance);
                    }
                }
                min_assignment
            }
            else {
                0
            }
        }

        assign(0, &workers, &bikes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::parse_pairs;
    use test_case::test_case;

    #[test_case("[[0,0],[2,1]]", "[[1,2],[3,3]]", 6)]
    #[test_case("[[0,0],[1,1],[2,0]]", "[[1,0],[2,2],[2,1]]", 4)]
    fn solve_brutforce(workers: &str, bikes: &str, expected: i32) {
        assert_eq!(
            expected,
            BruteforceSolution::assign_bikes(parse_pairs(workers), parse_pairs(bikes))
        );
    }
}
