use crate::util::linked_list::ListNode;
pub struct Solution1;
pub struct Solution2;

impl Solution1 {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut result_tail = &mut result;
        let (mut g_head, mut g_count) = (None, 0);

        while let Some(mut node) = head {
            head = node.next.take();
            if g_count % k == 0 {
                if let Some(group) = g_head.replace(node) {
                    _ = result_tail.insert(group);
                    while let Some(node) = result_tail {
                        result_tail = &mut node.next;
                    }
                }
            }
            else {
                g_head.insert(node).next = g_head.take();
            }
            g_count += 1;
        }
        if g_count % k != 0 {
            let mut tail_head = g_head.take();
            while let Some(mut node) = tail_head {
                tail_head = node.next.take();
                g_head.insert(node).next = g_head.take();
            }
        }
        if let Some(tail) = g_head {
            _ = result_tail.insert(tail);
        }

        result
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