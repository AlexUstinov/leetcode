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
use std::rc::Rc;
use std::cell::RefCell;
use crate::util::binary_tree::TreeNode;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut level = vec![];
        let mut queue = std::collections::VecDeque::new();
        if root.is_some() {
            queue.push_back(root);
            queue.push_back(None);
        }
        let mut is_forward = true;
        while let Some(node) = if is_forward { queue.pop_front() } else { queue.pop_back() } {
            if let Some(node) = node {
                let TreeNode { val, ref left, ref right } = &*node.borrow();
                level.push(*val);
                if is_forward {
                    for node in [left, right].into_iter().filter(|node| node.is_some()) {
                        queue.push_back(node.clone());    
                    }
                }
                else {
                    for node in [right, left].into_iter().filter(|node| node.is_some()) {
                        queue.push_front(node.clone());
                    }
                }
            }
            else {
                if !queue.is_empty() {
                    is_forward ^= true;
                    if is_forward {
                        queue.push_back(None);
                    }
                    else {
                        queue.push_front(None);
                    }
                }
                ans.push(level.clone());
                level.clear();
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::Solution;
    use crate::util::binary_tree::TreeNode;
    use std::rc::Rc;
    use std::cell::RefCell;
    

    #[test_case(TreeNode::from_str("[3,9,20,null,null,15,7]"), vec![vec![3],vec![20,9],vec![15,7]])]
    #[test_case(TreeNode::from_str("[]"), vec![])]
    fn solve(root: Option<Rc<RefCell<TreeNode>>>, expected: Vec<Vec<i32>>) {
        assert_eq!(expected, Solution::zigzag_level_order(root));    
    }
}