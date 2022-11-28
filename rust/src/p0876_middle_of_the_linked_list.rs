#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

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
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(1, 1)]
    #[test_case(12, 2)]
    #[test_case(1234, 3)]
    #[test_case(12345, 3)]
    fn solve(num: i32, expected: i32) {
        let l = convert_to_linked_list(num);
        assert_eq!(expected, Solution::middle_node(l).unwrap().val);
    }

    fn convert_to_linked_list(mut num: i32) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));
        while num > 0 {
            head.next = Some(Box::new(ListNode {
                val: num % 10,
                next: head.next,
            }));
            num /= 10;
        }
        head.next
    }
}
