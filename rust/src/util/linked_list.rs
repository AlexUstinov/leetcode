#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_num(mut num: i32) -> Option<Box<ListNode>> {
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

    pub fn from_num_reversed(mut num: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut tail_ref = &mut dummy_head;
        while num > 0 {
            if let Some(node) = tail_ref {
                node.next = Some(Box::new(ListNode::new(num % 10)));
                tail_ref = &mut node.next;
            }
            num /= 10;
        }
        dummy_head.unwrap().next
    }

    pub fn to_num(mut head: &Option<Box<ListNode>>) -> i32 {
        let mut num = 0;
        while let Some(node) = head {
            num *= 10;
            num += node.val;
            head = &node.next;
        }
        num
    }

    pub fn to_num_reversed(mut head: &Option<Box<ListNode>>) -> i32 {
        let (mut num, mut multiple) = (0, 1);
        while let Some(node) = head {
            num += node.val * multiple;
            multiple *= 10;
            head = &node.next;
        }
        num
    }
}