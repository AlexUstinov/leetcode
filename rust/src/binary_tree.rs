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
            match nodes.pop_front().expect("Queue must not be empty.") {
                Some(node) => {
                    let node = node.borrow();
                    result.push(Some(node.val));
                    val_count -= 1;
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
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::TreeNode;

    #[test]
    fn to_vec_method_works_correctly_1()
    {
        let numbers = vec![Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)];
        let root = TreeNode::from_vec(&numbers);
        assert_eq!(numbers, TreeNode::to_vec(&root.unwrap()));
    }

    #[test]
    fn to_vec_method_works_correctly_2()
    {
        let numbers = vec![Some(10), Some(5), Some(15), Some(3), Some(7), Some(18)];
        let root = TreeNode::from_vec(&numbers);
        assert_eq!(numbers, TreeNode::to_vec(&root.unwrap()));
    }

    #[test]
    fn to_vec_method_works_correctly_3()
    {
        let numbers = vec![Some(10), Some(5), Some(15), Some(3), Some(7), Some(18), Some(1), None, Some(6)];
        let root = TreeNode::from_vec(&numbers);
        assert_eq!(numbers, TreeNode::to_vec(&root.unwrap()));
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

    // [Theory]
    // [InlineData("[10,5,15,3,7,null,18]")]
    // [InlineData("[10,5,15,3,7,13,18,1,null,6]")]
    // [InlineData("[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]")]
    // public void ToFullStringMethodSerializesTreeProperly(string tree)
    // {
    //     var root = TreeNode.FromString(tree);
    //     Assert.Equal(tree, root!.ToFullString());
    // }

}