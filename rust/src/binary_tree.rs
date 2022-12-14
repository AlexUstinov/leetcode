use std::{cell::RefCell, collections::VecDeque, rc::Rc, fmt, iter};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }

    pub fn str_to_vec(values: &str) -> Vec<Option<i32>> {
        let trim_pat: &[_] = &[' ', '[', ']'];
        match values.trim_matches(trim_pat) {
            "" => Vec::new(),
            other => other.split(',')
                .map(|token| match token.trim() {
                    "" | "null" => None,
                    other => Some(other.parse::<i32>().unwrap())
                })
                .collect()
        }            
    }

    pub fn from_str(values: &str) -> Option<Rc<RefCell<TreeNode>>> {
        Self::from_vec(&Self::str_to_vec(values))
    }

    pub fn from_vec(values: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() {
            return None;
        }
        let root = Some(Rc::new(RefCell::new(TreeNode::new(values[0].expect("Root must have a value.")))));
        let mut parents = VecDeque::new();
        parents.push_back(root.as_ref().unwrap().clone());
        let mut is_left = true;
        for val in values.iter().skip(1) {
            let node = val.map_or(None, |val| Some(Rc::new(RefCell::new(TreeNode::new(val)))));
            let node_clone = node.clone();
            if is_left {
                let parent_ref = parents.front().expect("Parent node must exist in the queue.");
                let mut parent = parent_ref.as_ref().borrow_mut();
                parent.left = node;
            }
            else {
                let parent_ref = parents.pop_front().expect("Parent node must exist in the queue.");
                let mut parent = parent_ref.as_ref().borrow_mut();
                parent.right = node;
            }
            if let Some(node_rc) = &node_clone {
                parents.push_back(node_rc.clone());
            }
            is_left ^= true;
        }
        root
    }

    pub fn to_str(root: &Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = String::new();
        result.push('[');
        if let Some(root) = root {
            for num in root.borrow().iter_bfs() {
                match num {
                    None => result.push_str("null,"),
                    Some(num) => {
                        result.push_str(&num.to_string());
                        result.push(',');
                    }
                }
            }
        }
        let len = result.len();
        if len > 1 {
            result.truncate(len - 1);
        }
        result.push(']');
        result
    }

    pub fn to_vec(&self) -> Vec<Option<i32>> {
        let mut result = Vec::with_capacity(15);
        result.extend(self.iter_bfs());
        result
    }

    fn iter_bfs(&self) -> impl Iterator<Item=Option<i32>> + '_ {
        fn process_node(node: Option<&TreeNode>, nodes: &mut VecDeque<Option<Rc<RefCell<TreeNode>>>>, val_count: &mut i32) -> Option<i32> {
            if let Some(node) = node {
                nodes.extend([node.left.clone(), node.right.clone()].into_iter());
                *val_count += node.left.as_ref().map_or(0, |_| 1) + node.right.as_ref().map_or(0, |_| 1) - 1                    
            }
            node.map(|node| node.val)
        }
        let mut nodes = VecDeque::with_capacity(8);
        let mut val_count = 1;
        iter::from_fn(move || {
            if val_count==0 {
                return None;
            }
            Some(match nodes.pop_front() {
                None => process_node(Some(self), &mut nodes, &mut val_count),
                Some(None) => process_node(None, &mut nodes, &mut val_count),
                Some(node) => process_node(Some(&node.unwrap().borrow()), &mut nodes, &mut val_count),
            })
        })
    }
}

impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const MAX_COUNT: i32 = 15;
        let mut result = String::new();
        let mut count = 0;
        result.push('[');
        for num in self.iter_bfs() {
            count += 1;
            if count > MAX_COUNT {
                result.push_str("...,");
                break;
            }
            match num {
                None => result.push_str("null,"),
                Some(num) => {
                    result.push_str(&num.to_string());
                    result.push(',');
                }
            }
        }
        // We always have trailing comma character because we have at least self value in the list
        result.truncate(result.len() - 1);
        result.push(']');
        
        f.write_str(&result)
    }
}

#[cfg(test)]
mod tests {
    use super::TreeNode;
    use test_case::test_case;

    #[test_case(vec![Some(10), Some(5), Some(15), Some(3), Some(7), Some(18)])]
    #[test_case(vec![Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)])]
    #[test_case(vec![Some(10), Some(5), Some(15), Some(3), Some(7), Some(18), Some(1), None, Some(6)])]
    fn to_vec_method_works_correctly(numbers: Vec<Option<i32>>)
    {
        let root = TreeNode::from_vec(&numbers);
        assert_eq!(numbers, root.map(|node| node.borrow().to_vec()).unwrap());
    }

    #[test]
    fn display_fmt_doesnt_truncate_tree_with_less_than_16_nodes()
    {
        const TREE: &str = "[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]";
        let root = TreeNode::from_str(TREE);
        assert_eq!(TREE, root.map(|node| node.borrow().to_string()).unwrap());
    }

    #[test]
    fn display_fmt_truncates_trees_with_over_15_nodes()
    {
        const TREE: &str = "[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]";
        const TRUNCATED_TREE: &str = "[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,...]";
        let root = TreeNode::from_str(TREE);
        assert_eq!(TRUNCATED_TREE, root.map(|node| node.borrow().to_string()).unwrap());
    }

    #[test_case("[10,5,15,3,7,null,18]")]
    #[test_case("[10,5,15,3,7,13,18,1,null,6]")]
    #[test_case("[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]")]
    fn to_str_method_serializes_tree_properly(tree: &str) {
        let root = TreeNode::from_str(tree);
        assert_eq!(tree, TreeNode::to_str(&root));
    }

    #[test]
    fn to_str_method_serializes_empty_tree_properly() {
        const TREE: &str = "[]";
        let root = TreeNode::from_str(TREE);
        assert_eq!(TREE, TreeNode::to_str(&root));
    }
}
