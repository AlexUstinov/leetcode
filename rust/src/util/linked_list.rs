use std::{iter, fmt};

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

    pub fn from_str(values: &str) -> Option<Box<ListNode>> {
        Self::from_vec(&super::parse_values(values))
    }

    pub fn from_vec(values: &[i32]) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        for num in values.iter().rev() {
            head.next = Some(Box::new(ListNode {
                val: *num,
                next: head.next,
            }));
        }
        head.next
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

    pub fn to_str(head: &Option<Box<ListNode>>) -> String {
        let mut result = String::new();
        result.push('[');
        if let Some(root) = head {
            for num in root.iter_vals() {
                result.push_str(&num.to_string());
                result.push(',');
            }
        }
        let len = result.len();
        if len > 1 {
            result.truncate(len - 1);
        }
        result.push(']');
        result
    }

    pub fn to_vec(&self) -> Vec<i32> {
        let mut result = Vec::with_capacity(15);
        result.extend(self.iter_vals());
        result
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

    fn iter_vals(&self) -> impl Iterator<Item=i32> + '_ {
        let mut next: Option<&Option<Box<ListNode>>> = None;
        iter::from_fn(move || {
            match next {
                None => { next = Some(&self.next); Some(self.val) },
                Some(Some(node)) => { next = Some(&node.next); Some(node.val) },
                Some(None) => None
            }            
        })
    }

}

impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const MAX_COUNT: i32 = 15;
        let mut result = String::new();
        let mut count = 0;
        result.push('[');
        for num in self.iter_vals() {
            count += 1;
            if count > MAX_COUNT {
                result.push_str("...,");
                break;
            }
            result.push_str(&num.to_string());
            result.push(',');
        }
        // We always have trailing comma character because we have at least self value in the list
        result.truncate(result.len() - 1);
        result.push(']');
        
        f.write_str(&result)
    }
}