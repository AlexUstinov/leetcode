use crate::linked_list::ListNode;
pub struct Solution;

impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn link_node(tail: &mut Option<Box<ListNode>>, node: Box<ListNode>) -> &mut Option<Box<ListNode>> {
            let tail = tail.as_deref_mut().expect("Tail should point to the existing node.");
            tail.next = Some(node);
            &mut tail.next
        }
        let (mut odd, mut even) = (Some(Box::new(ListNode::new(0))), Some(Box::new(ListNode::new(0))));
        let (mut odd_tail, mut even_tail) = (&mut odd, &mut even);
        let mut is_odd = false;
        while let Some(mut node) = head {
            head = node.next;
            node.next = None;
            is_odd ^= true;
            match is_odd {
                true => odd_tail = link_node(odd_tail, node),
                false => even_tail = link_node(even_tail, node)
            }
        }

        if let Some(even_head) = even?.next {
            odd_tail.as_deref_mut().unwrap().next = Some(even_head);
        }

        odd?.next
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;
    use crate::linked_list::ListNode;

    use super::Solution;

    #[test_case(12345, 13524)]
    fn solve(num: i32, expected: i32) {
        let head = ListNode::from_num(num);
        assert_eq!(expected, ListNode::to_num(&Solution::odd_even_list(head)));
    }

}