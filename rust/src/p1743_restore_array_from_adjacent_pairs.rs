use std::collections::{HashSet, HashMap};

pub struct Solution;

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(adjacent_pairs.len()+1);

        let mut endings = HashSet::new();
        let mut adjacent = HashMap::new();
        for pair in adjacent_pairs {
            for el in pair.iter().copied() {
                if !endings.remove(&el) {
                    endings.insert(el);
                }
            }
            adjacent.entry(pair[0]).or_insert_with(Vec::new).push(pair[1]);
            adjacent.entry(pair[1]).or_insert_with(Vec::new).push(pair[0]);
        }
        let mut endings_iter = endings.into_iter();
        let mut item = endings_iter.next().unwrap();
        let last = endings_iter.next().unwrap();
        adjacent.remove(&last);
        while let Some(adj_items) = adjacent.get(&item) {
            for i in 0..adj_items.len() {
                if adj_items.get(i)==ans.last() {
                    continue;
                }
                ans.push(item);
                item = adj_items[i];
                break;
            }
        }
        ans.push(last);

        ans
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;
    use crate::util::parse_pairs;

    #[test_case(parse_pairs("[[2,1],[3,4],[3,2]]"), vec![1,2,3,4])]
    fn solve(adjacent_pairs: Vec<Vec<i32>>, expected: Vec<i32>) {
        let result = Solution::restore_array(adjacent_pairs);
        let mut rev_result = result.clone();
        rev_result.reverse();
        assert!(result==expected || rev_result==expected);
    }
}