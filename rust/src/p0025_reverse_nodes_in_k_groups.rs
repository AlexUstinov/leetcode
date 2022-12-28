use crate::util::linked_list::ListNode;
pub struct Solution1;
pub struct Solution2;

impl Solution1 {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut result = None;
            while let Some(mut node) = head {
                head = node.next.take();
                result.insert(node).next = result.take();
            }
            result
        }
        fn link_node(tail: &mut Option<Box<ListNode>>, node: Option<Box<ListNode>>) -> &mut Option<Box<ListNode>> {
            let tail = tail.as_deref_mut().expect("Tail should point to the existing node.");
            tail.next = node;
            &mut tail.next
        }

        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut result_tail = &mut dummy_head;
        let mut g_head = None;
        let (mut g_tail, mut g_count) = (&mut g_head, 0);

        while let Some(mut node) = head {
            head = node.next.take();
            if g_count % k == 0 {
                let mut g = reverse_list(g_head);
                while let Some(mut node) = g {
                    g = node.next.take();
                    result_tail = link_node(result_tail, Some(node));
                }
                g_head = Some(node);
                g_tail = &mut g_head;
            }
            else {
                g_tail = link_node(g_tail, Some(node));
            }
            g_count += 1;
        }
        if g_count % k == 0 {
            g_head = reverse_list(g_head);
        }
        link_node(result_tail, g_head);

        dummy_head.expect("Dummy head is never None.").next
    }
}

impl Solution2 {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut node = &mut head;
        for _ in 0..k {
            if let Some(n) = node {
                node = &mut n.next;
            } else {
                return head;
            }
        }
        let mut ret = Self::reverse_k_group(node.take(), k);
        while let Some(h) = head.take() {
            ret = Some(Box::new(ListNode {
                val: h.val,
                next: ret,
            }));
            head = h.next;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::util::linked_list::ListNode; 
    use super::*;

    #[test_case("[1,2]", 2, "[2,1]")]
    #[test_case("[1,2,3,4]", 2, "[2,1,4,3]")]
    #[test_case("[1,2,3,4,5]", 2, "[2,1,4,3,5]")]
    #[test_case("[1,2,3,4,5]", 3, "[3,2,1,4,5]")]
    fn solve(list: &str, k: i32, expected: &str) {
        assert_eq!(expected, ListNode::to_str(&Solution1::reverse_k_group(ListNode::from_str(list), k)));
        assert_eq!(expected, ListNode::to_str(&Solution2::reverse_k_group(ListNode::from_str(list), k)));
    }
}