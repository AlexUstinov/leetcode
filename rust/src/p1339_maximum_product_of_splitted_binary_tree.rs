use crate::binary_tree::TreeNode;

use std::cmp;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;
impl Solution {
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs<T, G>(node: &Option<Rc<RefCell<TreeNode>>>, func: &G, default: T) -> T
            where
                G: Fn(i64, T, T) -> T,
                T: Copy  {
            match node.as_deref() {
                Some(node) => {
                    let node = &*node.borrow();
                    func(node.val as i64, dfs(&node.left, func, default), dfs(&node.right, func, default))
                },
                None => default
            }
        }
        let tree_sum = dfs(&root, &|val: i64, left: i64, right: i64| val + left + right, 0i64);
        let (_, min_diff) = dfs(&root,&|val: i64, left: (i64, i64), right: (i64, i64)| {
            let sum = val + left.0 + right.0;
            let diff: i64 = tree_sum - 2 * sum;
            let min_diff = cmp::min(cmp::min(diff.abs(), left.1), right.1);
            (sum, min_diff)
        }, (0, i64::MAX));
        let subtree_sum = (tree_sum - min_diff) / 2;
        const MODULO: i64 = 1000000007;
        (((subtree_sum % MODULO) * ((tree_sum - subtree_sum) % MODULO)) % MODULO).try_into()
            .expect("After modulo division the value must fit")
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::binary_tree::TreeNode;
    use super::Solution;

    #[test_case("[1,2,3,4,5,6]", 110)]
    #[test_case("[1,null,2,3,4,null,null,5,6]", 90)]
    fn solve(tree: &str, expected: i32) {
        assert_eq!(expected, Solution::max_product(TreeNode::from_str(tree)));
    }
}