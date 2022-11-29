use crate::linked_list::ListNode;
pub struct Solution;

impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut odd, mut even) = (None, None);
        let (mut odd_tail, mut even_tail) = (&mut odd, &mut even);
        let mut is_odd = true;
        while let Some(mut node) = head {
            head = node.next;
            node.next = None;
            match is_odd {
                true => odd_tail = &mut odd_tail.insert(node).next,
                false => even_tail = &mut even_tail.insert(node).next
            }
            is_odd ^= true;
        }

        if let Some(even_head) = even {
            _ = odd_tail.insert(even_head);
        }

        odd
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