use crate::util::binary_tree::TreeNode;

use std::cmp;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;
const MOD: i64 = 1_000_000_007;
impl Solution {
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs<T, G>(node: &Option<Rc<RefCell<TreeNode>>>, func: &G, default: T) -> T
            where G: Fn(i64, T, T) -> T, T: Copy
        {
            if let Some(node) = node {
                let node = node.borrow();
                return func(node.val as i64, dfs(&node.left, func, default), dfs(&node.right, func, default));
            }
            default
        }
        let tree_sum = dfs(&root, &|val: i64, left: i64, right: i64| val + left + right, 0i64);
        let (_, min_diff) = dfs(&root,&|val: i64, left: (i64, i64), right: (i64, i64)| {
            let sum = val + left.0 + right.0;
            (sum, cmp::min((tree_sum - 2 * sum).abs(), cmp::min(left.1, right.1)))
        }, (0, i64::MAX));
        let subtree_sum = (tree_sum - min_diff) / 2;
        (((subtree_sum % MOD) * ((tree_sum - subtree_sum) % MOD)) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::util::binary_tree::TreeNode;
    use super::Solution;

    #[test_case("[1,2,3,4,5,6]", 110)]
    #[test_case("[1,null,2,3,4,null,null,5,6]", 90)]
    fn solve(tree: &str, expected: i32) {
        assert_eq!(expected, Solution::max_product(TreeNode::from_str(tree)));
    }
}