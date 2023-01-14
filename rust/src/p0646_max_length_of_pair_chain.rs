pub struct Solution1;

enum Action {Explore(usize, usize), Restore(usize, usize)}
impl Solution1 {
    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        fn is_predecessor(a: &[i32], b: &[i32]) -> bool {
            a[1] < b[0]
        }
        let mut g = vec![vec![]; pairs.len()+1];
        g[pairs.len()].extend(0..pairs.len());
        for (i, a) in pairs.iter().enumerate() {
            for (j, b) in pairs.iter().take(i).enumerate() {
                if is_predecessor(a, b) {
                    g[i].push(j);
                }
                if is_predecessor(b, a) {
                    g[j].push(i)
                }
            }
        }

        // dp[node] is length of the longest path from the node to a DAG leaf
        let mut dp = vec![None; pairs.len()+1];
        let mut stack = vec![Action::Explore(pairs.len(), pairs.len())];
        while let Some(action) = stack.pop() {
            match action {
                Action::Explore(node, parent) => {
                    if node!=parent {
                        stack.push(Action::Restore(node, parent));
                    }
                    if dp[node].is_none() {
                        let children = &g[node];
                        if !children.is_empty() {
                            stack.extend(children.iter().map(|&child| Action::Explore(child, node)));
                        }
                        else {
                            dp[node] = Some(0);
                        }
                    }
                },
                Action::Restore(node, parent) => {
                    dp[parent] = dp[parent].zip(dp[node])
                        .map(|(parent_depth, node_depth)| parent_depth.max(node_depth + 1))
                        .or_else(||dp[node].map(|depth| depth + 1));
                }
            }
        }

        dp[pairs.len()].unwrap()
    }
}

pub struct Solution2;

impl Solution2 {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let mut dp = vec![0; pairs.len()];
        for (i, tail) in pairs.iter().enumerate() {
            dp[i] = dp.iter().take(i).enumerate().filter(|(j, _)| pairs[*j][1] < tail[0]).map(|(_, len)| *len).max().unwrap_or(0) + 1;
        }
        *dp.iter().max().unwrap()
    }    
}

pub struct Solution3;

impl Solution3 {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        pairs.iter().fold((0, i32::MIN), |(len, seq_end), el| if el[0]>seq_end { (len+1, el[1]) } else { (len, seq_end) }).0
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;
    use crate::util::parse_pairs;

    #[test_case(parse_pairs("[[1,2],[7,8],[4,5]]"), 3)]
    fn solve1(pairs: Vec<Vec<i32>>, expected: i32) {
        assert_eq!(expected, Solution1::find_longest_chain(pairs))
    }

    #[test_case(parse_pairs("[[1,2],[7,8],[4,5]]"), 3)]
    fn solve2(pairs: Vec<Vec<i32>>, expected: i32) {
        assert_eq!(expected, Solution2::find_longest_chain(pairs))
    }

    #[test_case(parse_pairs("[[1,2],[7,8],[4,5]]"), 3)]
    fn solve3(pairs: Vec<Vec<i32>>, expected: i32) {
        assert_eq!(expected, Solution3::find_longest_chain(pairs))
    }

}