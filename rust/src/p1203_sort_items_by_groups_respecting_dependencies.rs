use std::collections::{HashMap, HashSet};
pub struct Solution;
impl Solution {
    pub fn sort_items(n: i32, mut m: i32, mut group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
        fn topological_sort(mut in_degrees: Vec<usize>, graph: HashMap<i32, HashSet<i32>>) -> Vec<i32> {
            let mut stack = Vec::new();
            stack.extend(in_degrees.iter().copied().enumerate().filter(|&(_, degree)| degree==0).map(|(i, _)| i as i32));

            let mut ans = Vec::with_capacity(in_degrees.len());
            while let Some(item) = stack.pop() {
                ans.push(item);
                for next in graph.get(&item).into_iter().flatten().copied().map(|next| next as usize) {
                    in_degrees[next] -= 1;
                    if in_degrees[next] == 0 {
                        stack.push(next as i32);
                    }
                }
            }

            if ans.len()==in_degrees.len() { ans } else { vec![] }
        }

        // Assign group ids
        for group_id in group.iter_mut() {
            if *group_id==-1 {
                *group_id = m;
                m += 1;
            }
        }

        let mut item_graph = HashMap::new();
        let mut item_degrees = vec![0; n as usize];

        let mut group_graph = HashMap::new();
        let mut group_degrees = vec![0; m as usize];

        for (item, prev_items) in before_items.into_iter().enumerate() {
            item_degrees[item] = prev_items.len();
            let item_group = group[item];
            for prev_item in prev_items {
                let prev_item_group = group[prev_item as usize];
                item_graph.entry(prev_item).or_insert_with(HashSet::new).insert(item as i32);
                if item_group!=prev_item_group && group_graph.entry(prev_item_group).or_insert_with(HashSet::new).insert(item_group) {
                    group_degrees[item_group as usize] += 1;
                }
            }
        }

        let mut grouped_items = HashMap::new();
        for item in topological_sort(item_degrees, item_graph) {
            grouped_items.entry(group[item as usize]).or_insert_with(Vec::new).push(item);
        }

        let mut ans = Vec::new();
        for group_id in topological_sort(group_degrees, group_graph) {
            for item in grouped_items.get(&group_id).into_iter().flatten().copied() {
                ans.push(item);
            }
        }
 
        ans
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;

    // #[test_case("[1,2,3,4,5,6]", 110)]
    #[test_case(8, 2, vec![-1,-1,1,0,0,1,0,-1], vec![vec![],vec![6],vec![5],vec![6],vec![3,6],vec![],vec![],vec![]], vec![6,3,4,1,5,2,0,7])]
    fn solve(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>, mut expected: Vec<i32>) {
        let mut solution = Solution::sort_items(n, m, group, before_items);
        solution.sort_unstable();
        expected.sort_unstable();
        assert_eq!(expected, solution);
    }

}