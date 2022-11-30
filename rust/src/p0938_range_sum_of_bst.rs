use std::rc::Rc;
use std::cell::RefCell;
use crate::binary_tree::TreeNode;
pub struct Solution;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        fn range_sum(root: &Option<Rc<RefCell<TreeNode>>>, low: &i32, high: &i32) -> i32 {
            match root {
                Some(root) => match &*root.borrow() {
                    TreeNode {val, left, right} if val >= low && val <= high => *val + range_sum(left, low, high) + range_sum(right, low, high),
                    TreeNode {val, right, ..} if val < low => range_sum(right, low, high),
                    TreeNode {val, left, ..} if val > high => range_sum(left, low, high),
                    _ => 0
                },
                None => 0
            }
        }
        range_sum(&root, &low, &high)
    }
}

#[cfg(test)]
mod test {
    use crate::binary_tree::TreeNode;
    use test_case::test_case;
    use super::Solution;

    #[test_case("[10,5,15,3,7,null,18]", 7, 15, 32)]
    #[test_case("[10,5,15,3,7,13,18,1,null,6]", 6, 10, 23)]
    fn solve(tree: &str, low: i32, high: i32, expected: i32) {
        let tree = TreeNode::from_str(tree);
        assert_eq!(expected, Solution::range_sum_bst(tree, low, high));
    }
}