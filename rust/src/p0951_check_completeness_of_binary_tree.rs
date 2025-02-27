pub struct Solution;

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use crate::util::binary_tree::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut max_depth = {
            let mut depth = 0;
            let mut next = root.clone();
            while let Some(node) = next {
                let TreeNode { left, .. } = &*node.borrow();
                next = left.clone();
                depth += 1;
            }
            depth
        };
        let mut stack = vec![(root, 1)];
        let mut is_lvl_tail = false;
        while let Some((node, depth)) = stack.pop() {
            if let Some(node) = node.as_ref() {
                let TreeNode { left, right, .. } = &*node.borrow();
                if depth == max_depth {
                    if left.is_some() || right.is_some() {
                        return false;
                    }
                    continue;
                }
                if !is_lvl_tail && depth == max_depth - 1 && right.is_none() {
                    is_lvl_tail = true;
                    max_depth -= 1;
                    if left.is_some() {
                        stack.push((left.clone(), depth));
                    }
                    continue;
                }

                stack.push((right.clone(), depth + 1));
                stack.push((left.clone(), depth + 1));
            }
            else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::util::binary_tree::*;

    #[test_case(TreeNode::from_str("[1]"), true)]
    #[test_case(TreeNode::from_str("[1,2]"), true)]
    #[test_case(TreeNode::from_str("[1,2,3]"), true)]
    #[test_case(TreeNode::from_str("[1,2,3,4]"), true)]
    #[test_case(TreeNode::from_str("[1,2,3,4,5]"), true)]
    #[test_case(TreeNode::from_str("[1,2,3,4,5,6]"), true)]
    #[test_case(TreeNode::from_str("[1,2,3,null,5]"), false)]
    #[test_case(TreeNode::from_str("[1,2,3,null,null,6]"), false)]
    #[test_case(TreeNode::from_str("[1,2,3,null,null,null,7]"), false)]
    fn solve(root: Option<Rc<RefCell<TreeNode>>>, expected: bool) {
        assert_eq!(expected, Solution::is_complete_tree(root));
    }
}