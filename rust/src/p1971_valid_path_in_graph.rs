use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution;

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let g = edges.iter()
            .flat_map(|e| [[e[0], e[1]], [e[1], e[0]]])
            .fold(HashMap::<i32, Vec<i32>>::with_capacity(2*edges.len()), |mut g, edge| {
                match g.get_mut(&edge[0]) {
                    Some(list) => { list.push(edge[1]); g },
                    None => { g.insert(edge[0], vec![edge[1]]); g }
                }
            });
        if source != destination && g.get(&source).is_none() {
            return false;
        }
        let size = n as usize;
        let mut visited = HashSet::with_capacity(size);
        let mut queue = VecDeque::with_capacity(size);
        queue.push_back(source);
        while let Some(node) = queue.pop_front() {
            if node==destination {
                return true;
            }
            if let Some(adj_nodes) = g.get(&node) {
                queue.extend(adj_nodes.iter().filter(|&adj_node| visited.insert(*adj_node)))
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::util::parse_pairs;
    use super::Solution;

    #[test_case(3, parse_pairs("[[0,1],[1,2],[2,0]]"), 0, 2, true)]
    #[test_case(4, parse_pairs("[[0,1],[1,2],[2,0]]"), 4, 2, false)]
    #[test_case(4, parse_pairs("[[0,1],[1,2],[2,0]]"), 0, 4, false)]
    #[test_case(6, parse_pairs("[[0,1],[0,2],[3,5],[5,4],[4,3]]"), 0, 5, false)]
    fn solve(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32, expected: bool) {
        assert_eq!(expected, Solution::valid_path(n, edges, source, destination));
    }
}