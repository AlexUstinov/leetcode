use crate::linked_list::ListNode;
pub struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut fast, mut slow) = (&head, &head);
        while let Some(ListNode { next: Some(node), .. }) = fast.as_deref() {
            fast = &node.next;
            slow = &slow.as_ref().unwrap().next;
        }
        slow.clone()
    }

    pub fn middle_node_counter(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut count = 0;
        let mut pointer = &head;
        while let Some(node) = pointer {
            pointer = &node.next;
            count += 1;
        }
        count >>= 1;
        while count > 0 {
            if let Some(node) = head {
                head = node.next;
            }
            count -= 1;
        }
        head
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(1, 1)]
    #[test_case(12, 2)]
    #[test_case(1234, 34)]
    #[test_case(12345, 345)]
    fn solve(num: i32, expected: i32) {
        let l = ListNode::from_num(num);
        assert_eq!(expected, ListNode::to_num(&Solution::middle_node(l)));
        let l = ListNode::from_num(num);
        assert_eq!(expected, ListNode::to_num(&Solution::middle_node_counter(l)));
    }
}
