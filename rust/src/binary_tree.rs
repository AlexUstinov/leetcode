use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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
                "" | "null" => None,
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

    pub fn to_str(root: &Rc<RefCell<TreeNode>>) -> String {
        let mut result = String::new();
        result.push('[');
        for num in &root.borrow().to_vec() {
            match num {
                None => result.push_str("null,"),
                Some(num) => {
                    result.push_str(&num.to_string());
                    result.push(',');
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
        fn process_node(node: Option<&TreeNode>, result: &mut Vec<Option<i32>>, nodes: &mut VecDeque<Option<Rc<RefCell<TreeNode>>>>, val_count: &mut i32) {
            result.push(node.map(|node| node.val));
            if let Some(node) = node {
                nodes.extend([node.left.clone(), node.right.clone()].into_iter());
                *val_count += node.left.as_ref().map_or(0, |_| 1) + node.right.as_ref().map_or(0, |_| 1) - 1                    
            }
        }
        let (mut result, mut nodes) = (Vec::with_capacity(15), VecDeque::with_capacity(8));
        let mut val_count = 1;
        while val_count > 0 {
            match nodes.pop_front() {
                None => process_node(Some(self), &mut result, &mut nodes, &mut val_count),
                Some(None) => process_node(None, &mut result, &mut nodes, &mut val_count),
                Some(node) => process_node(Some(&node.unwrap().borrow()), &mut result, &mut nodes, &mut val_count),
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::TreeNode;
    use test_case::test_case;

    #[test]
    fn to_vec_method_works_correctly_1()
    {
        let numbers = vec![Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)];
        let root = TreeNode::from_vec(&numbers);
        assert_eq!(numbers, root.map(|node| node.borrow().to_vec()).unwrap());
    }

    #[test]
    fn to_vec_method_works_correctly_2()
    {
        let numbers = vec![Some(10), Some(5), Some(15), Some(3), Some(7), Some(18)];
        let root = TreeNode::from_vec(&numbers);
        assert_eq!(numbers, root.map(|node| node.borrow().to_vec()).unwrap());
    }

    #[test]
    fn to_vec_method_works_correctly_3()
    {
        let numbers = vec![Some(10), Some(5), Some(15), Some(3), Some(7), Some(18), Some(1), None, Some(6)];
        let root = TreeNode::from_vec(&numbers);
        assert_eq!(numbers, root.map(|node| node.borrow().to_vec()).unwrap());
    }

    #[test_case(vec![Some(10), Some(5), Some(15), Some(3), Some(7), Some(18)])]
    #[test_case(vec![Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)])]
    #[test_case(vec![Some(10), Some(5), Some(15), Some(3), Some(7), Some(18), Some(1), None, Some(6)])]
    fn to_vec_2_method_works_correctly(numbers: Vec<Option<i32>>)
    {
        let root = TreeNode::from_vec(&numbers);
        assert_eq!(numbers, root.map(|node| node.borrow().to_vec()).unwrap());
    }

    // [Fact]
    // public void ToStringMethodDoesntTruncateTreeWithLessThan16Nodes()
    // {
    //     const string tree = "[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]";
    //     var root = TreeNode.FromString(tree);
    //     Assert.Equal(tree, root!.ToString());
    // }

    // [Fact]
    // public void ToStringMethodTruncatesTreesWithOver15Nodes()
    // {
    //     const string tree = "[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]";
    //     const string truncatedTree = "[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,...]";
    //     var root = TreeNode.FromString(tree);
    //     Assert.Equal(truncatedTree, root!.ToString());
    // }

    #[test_case("[10,5,15,3,7,null,18]")]
    #[test_case("[10,5,15,3,7,13,18,1,null,6]")]
    #[test_case("[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]")]
    fn to_str_method_serializes_tree_properly(tree: &str) {
        let root = TreeNode::from_str(tree);
        assert_eq!(tree, TreeNode::to_str(&root.unwrap()));
    }
}
