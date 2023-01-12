pub struct Solution;

enum Action { Explore(usize, usize), Restore(usize, usize) }
impl Solution {
    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let n = vals.len() as usize;
        let g = edges.iter().fold(vec![vec![]; n], |mut g, e| { let (a, b) = (e[0] as usize, e[1] as usize); g[a].push(b); g[b].push(a); g });
        let mut stack = vec![Action::Explore(0, 0)];
        let (mut down_path_counts, mut path_count) = (vec![Vec::<HashMap<i32, i32>>::new(); n], 0);
        while let Some(action) = stack.pop() {
            match action {
                Action::Explore(node, parent) => {
                    stack.push(Action::Restore(node, parent));
                    for &child in g[node].iter().filter(|&&nbr| nbr!=parent) {
                        stack.push(Action::Explore(child, node));
                    }
                },
                Action::Restore(node, parent) => {
                    let (node_val, parent_val) = (vals[node], vals[parent]);
                    let path_counts = &down_path_counts[node];
                    let mut unique_ends = HashMap::new();
                    path_count += 1;
                    for i in 0..path_counts.len() {
                        let a = &path_counts[i];
                        if let Some(count) = a.get(&node_val) {
                            path_count += count;
                        }
                        for j in (i+1)..path_counts.len() {
                            let b = &path_counts[j];
                            for (key, a_cnt) in a.iter() {
                                if let Some(b_cnt) = b.get(key) {
                                    path_count += a_cnt * b_cnt;
                                }
                            }
                        }
                        for (&val, &a_cnt) in a.iter().filter(|(&val, _)| val >= parent_val) {
                            if let Some(cnt) = unique_ends.get_mut(&val) {
                                *cnt += a_cnt;
                            }
                            else {
                                unique_ends.insert(val, a_cnt);
                            }
                        }
                    }
                    if node_val >= parent_val {
                        if let Some(cnt) = unique_ends.get_mut(&node_val) {
                            *cnt += 1;
                        }
                        else {
                            unique_ends.insert(node_val, 1);
                        }
                    }
                    down_path_counts[parent].push(unique_ends);
                    down_path_counts[node].clear();
                }
            }
        }
        path_count
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;
    use crate::util::parse_pairs;

    
    #[test_case(vec![4,4,3,4,1,4,5], parse_pairs("[[0,1],[2,1],[3,2],[4,0],[5,2],[3,6]]"), 13)]
    #[test_case(vec![2,5,5,1,5,2,3,5,1,5], parse_pairs("[[0,1],[2,1],[3,2],[3,4],[3,5],[5,6],[1,7],[8,4],[9,7]]"), 20)]
    fn solve(vals: Vec<i32>, edges: Vec<Vec<i32>>, expected: i32) {
        assert_eq!(expected, Solution::number_of_good_paths(vals, edges));
    }
}