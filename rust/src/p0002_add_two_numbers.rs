use crate::linked_list::ListNode;
pub struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let default_node = Box::new(ListNode::new(0));
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut tail_ref = &mut dummy_head;
        let mut l1_ref = &l1;
        let mut l2_ref = &l2;
        let mut carry = 0;
        while l1_ref.is_some() || l2_ref.is_some() || carry > 0 {
            let l1_node = l1_ref.as_ref().unwrap_or(&default_node);
            let l2_node = l2_ref.as_ref().unwrap_or(&default_node);
            let sum = l1_node.val + l2_node.val + carry;
            match *tail_ref {
                None => unreachable!(),
                Some(ref mut tail_node) => {
                    tail_node.next = Some(Box::new(ListNode::new(sum % 10)));
                    tail_ref = &mut tail_node.next;
                }
            }
            l1_ref = &l1_node.next;
            l2_ref = &l2_node.next;
            carry = sum / 10;
        }
        dummy_head.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;
    use super::*;

    #[test_case(1, 2, 3)]
    #[test_case(1, 20, 21)]
    #[test_case(1, 999, 1000)]
    fn solve(num1: i32, num2: i32, expected: i32) {
        let l1 = ListNode::from_num_reversed(num1);
        let l2 = ListNode::from_num_reversed(num2);
        assert_eq!(ListNode::to_num_reversed(&Solution::add_two_numbers(l1, l2)), expected);
    }
}