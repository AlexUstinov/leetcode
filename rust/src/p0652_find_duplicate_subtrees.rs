pub struct Solution;

use crate::util::binary_tree::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn find_duplicate_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        enum Action { Explore(i32, Option<Rc<RefCell<TreeNode>>>), Restore(Option<Rc<RefCell<TreeNode>>>, i32, i32, i32, i32) }
        let mut ans = vec![];
        // child_struct_ids map is used to communicate between iterations
        // it delivers child node structure ids to the parent node
        let mut child_struct_ids = std::collections::HashMap::new();
        // maps structure to its ID
        let mut structure_id_map = std::collections::HashMap::new();
        // counters of different structures
        let mut counts = std::collections::HashMap::new();
        let mut id = 0;
        let mut id_gen = move || { id+=1; id };
        let mut stack = vec![Action::Explore(id_gen(), root)];
        while let Some(action) = stack.pop() {
            match action {
                Action::Explore(node_id, opt_node) => {
                    if let &Some(ref node) = &opt_node {
                        let &TreeNode { val, ref left, ref right } = &*node.borrow();
                        let (left_id, right_id) = (id_gen(), id_gen());
                        stack.extend([
                            Action::Restore(opt_node.clone(), val, node_id, left_id, right_id),
                            Action::Explore(right_id, right.clone()),
                            Action::Explore(left_id, left.clone())
                        ]);
                    }
                },
                Action::Restore(node, val, node_id, left_id, right_id) => {
                    // lookup for the left & right subtree structure ids
                    let l_struct_id = child_struct_ids.remove(&left_id).unwrap_or(0);
                    let r_struct_id = child_struct_ids.remove(&right_id).unwrap_or(0);
                    // define structure of the current node
                    let node_struct = (l_struct_id, val, r_struct_id);
                    // lookup for the structure id or create a new one
                    let node_struct_id = *structure_id_map.entry(node_struct).or_insert_with(|| id_gen());
                    // communicate structure id to the parent node
                    child_struct_ids.insert(node_id, node_struct_id);
                    // count how many times current structure has appeared
                    let struct_count = counts.entry(node_struct_id).or_insert(0);
                    if *struct_count == 1 {
                        // output result on first duplication
                        ans.push(node);
                    }
                    *struct_count += 1;
                }
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

    #[test_case(TreeNode::from_str("[2,2,2,3,null,3,null]"))]
    fn solve(root: Option<Rc<RefCell<TreeNode>>>) {
        assert!(!Solution::find_duplicate_subtrees(root).is_empty())
    }

}