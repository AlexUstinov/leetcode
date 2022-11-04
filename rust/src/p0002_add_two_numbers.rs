#![allow(dead_code)]

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!()        
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
        let l1 = convert_to_linked_list(num1);
        let l2 = convert_to_linked_list(num2);
        assert_eq!(convert_from_linked_list(Solution::add_two_numbers(l1, l2)), expected);
    }

    fn convert_to_linked_list(num1: i32) -> Option<Box<ListNode>> {
        todo!()
    }

    fn convert_from_linked_list(l: Option<Box<ListNode>>) -> i32 {
        todo!()
    }
}