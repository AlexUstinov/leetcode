use crate::binary_tree::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;
impl Solution {
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn to_leaf_seq(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>{
            fn build_leaf_seq(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
                if let Some(node) = node.as_deref() {
                    let node = &*node.borrow();
                    if node.left.is_none() && node.right.is_none() {
                        result.push(node.val);
                    }
                    build_leaf_seq(&node.left, result);
                    build_leaf_seq(&node.right, result);
                }
            }
            let mut result = Vec::new();
            build_leaf_seq(root, &mut result);
            result
        }
        let leaf_seq1 = to_leaf_seq(&root1);
        let leaf_seq2 = to_leaf_seq(&root2);

        leaf_seq1.eq(&leaf_seq2)
    }
}

#[cfg(test)]
mod test {
    use crate::binary_tree::TreeNode;
    use test_case::test_case;
    use super::Solution;

    #[test_case("[1,2,3]", "[1,3,2]", false)]
    #[test_case("[3,5,1,6,2,9,8,null,null,7,4]", "[3,5,1,6,7,4,2,null,null,null,null,null,null,9,8]", true)]
    fn solve(tree1: &str, tree2: &str, expected: bool) {
        let root1 = TreeNode::from_str(tree1);
        let root2 = TreeNode::from_str(tree2);
        assert_eq!(expected, Solution::leaf_similar(root1, root2));
    }
}