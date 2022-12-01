use std::{rc::Rc, cell::RefCell, collections::VecDeque};

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
        values
            .trim_matches(trim_pat)
            .split(',')
            .map(|token| match token.trim() {
                ""|"null" => None,
                other => Some(other.parse::<i32>().unwrap())
            })
            .collect()
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

    pub fn to_vec(root: &Rc<RefCell<TreeNode>>) -> Vec<Option<i32>> {
        let mut result = Vec::new();
        let mut nodes = VecDeque::new();
        nodes.push_back(Some(root.clone()));
        let mut val_count = 1;
        while val_count > 0 {
            let node = nodes.pop_front().expect("Queue must not be empty.");
            match node.as_deref() {
                Some(node) => {
                    let node = &*node.borrow();
                    result.push(Some(node.val));
                    if node.left.is_some() {
                        val_count += 1;
                    }
                    if node.right.is_some() {
                        val_count += 1;
                    }
                    nodes.push_back(node.left.clone());
                    nodes.push_back(node.right.clone());
                },
                None => result.push(None)
            }
            val_count -= 1;
        }
        result
    }
}