use crate::util::linked_list::ListNode;
pub struct Solution;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::util::linked_list::ListNode;
    use super::Solution;

    #[test_case("[1,2,3,4,5]", 2, "[2,1,4,3,5]")]
    #[test_case("[1,2,3,4,5]", 3, "[3,2,1,4,5]")]
    fn solve(list: &str, k: i32, expected: &str) {
        assert_eq!(expected, ListNode::to_str(&Solution::reverse_k_group(ListNode::from_str(list), k)))
    }
}