pub struct Solution;

enum Action { Explore { node: usize, parent: usize }, Restore { node: usize, prev_cnt: i32 } }
impl Solution {
    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let g = edges.iter().map(|e| [e[0] as usize, e[1] as usize]).fold(vec![Vec::<usize>::new(); n as usize], |mut g, e| { g[e[0]].push(e[1]); g[e[1]].push(e[0]); g });
        let labels = labels.as_bytes();
        let (mut result, mut stack) = (vec![0; n as usize], vec![Action::Explore { node: 0, parent: 0 }]);
        let mut counters = vec![0; (b'z' - b'a' + 1) as usize];
        while let Some(action) = stack.pop() {
            match action {
                Action::Explore { node, parent } => {
                    let label_idx = (labels[node] - b'a') as usize;
                    stack.push(Action::Restore { node, prev_cnt: counters[label_idx] });
                    counters[label_idx] += 1;
                    stack.extend(g[node].iter().filter(|&&nbr| nbr != parent).map(|&child| Action::Explore { node: child, parent: node }));
                },
                Action::Restore { node, prev_cnt } => result[node] = counters[(labels[node] - b'a') as usize] - prev_cnt
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;
    use crate::util::parse_pairs;

    #[test_case(4, parse_pairs("[[0,1],[1,2],[0,3]]"), String::from("bbbb"), vec![4,2,1,1])]
    #[test_case(5, parse_pairs("[[0,1],[0,2],[1,3],[0,4]]"), String::from("aabab"), vec![3,2,1,1,1])]
    #[test_case(7, parse_pairs("[[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]]"), String::from("abaedcd"), vec![2,1,1,1,1,1,1])]
    fn solve(n: i32, edges: Vec<Vec<i32>>, labels: String, expected: Vec<i32>) {
        assert_eq!(expected, Solution::count_sub_trees(n, edges, labels));
    }
}