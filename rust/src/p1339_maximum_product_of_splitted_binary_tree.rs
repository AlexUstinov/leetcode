use crate::binary_tree::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;
impl Solution {
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        
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