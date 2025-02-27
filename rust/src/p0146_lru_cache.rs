use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
pub struct ListNode {
    key: i32,
    val: i32,
    prev: Option<Rc<RefCell<ListNode>>>,
    next: Option<Rc<RefCell<ListNode>>>
}

impl ListNode {
    pub fn new(key: i32, val: i32) -> Self {
        Self { key, val, prev: None, next: None }
    }
}

pub struct LRUCache {
    capacity: usize,
    head: Option<Rc<RefCell<ListNode>>>,
    tail: Option<Rc<RefCell<ListNode>>>,
    map: HashMap<i32, Rc<RefCell<ListNode>>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    pub fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            head: None,
            tail: None,
            map: HashMap::new()
        }
    }
    
    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get_mut(&key) {
            let ListNode { val, prev, next, .. } = &mut *node.borrow_mut();
            let mut insert_as_head = false;
            if let Some(prev_node) = prev {
                let ListNode { next: prev_next, .. } = &mut *prev_node.borrow_mut();
                if let Some(next_node) = next.as_ref() {
                    let ListNode { prev: next_prev, ..} = &mut *next_node.borrow_mut();
                    _ = prev_next.insert(Rc::clone(next_node));
                    _ = next_prev.insert(Rc::clone(prev_node));
                } else {
                    *prev_next = None;
                    _ = self.tail.insert(Rc::clone(prev_node));
                }
                insert_as_head = true;
            }
            if insert_as_head {
                if let Some(head_node) = self.head.as_ref() {
                    let ListNode { prev: head_prev, .. } = &mut *head_node.borrow_mut();
                    _ = head_prev.insert(Rc::clone(node));
                }
                *next = self.head.take();
                _ = self.head.insert(Rc::clone(node));
                *prev = None;
            }

            return *val;
        }
        -1
    }
    
    pub fn put(&mut self, key: i32, value: i32) {
        if self.get(key) >= 0 {
            if let Some(node) = self.map.get_mut(&key) {
                let ListNode { val, .. } = &mut *node.borrow_mut();
                *val = value;
            }
        }
        else {
            let node = Rc::new(RefCell::new(ListNode::new(key, value)));
            if let Some(head) = self.head.as_ref() {
                let ListNode { prev: head_prev, .. } = &mut *head.borrow_mut();
                _ = head_prev.insert(Rc::clone(&node));
                let ListNode { next: node_next, .. } = &mut *node.borrow_mut();
                _ = node_next.insert(Rc::clone(head));
            }
            if self.tail.is_none() {
                self.tail = Some(Rc::clone(&node));
            }
            self.head = Some(Rc::clone(&node));
            self.map.insert(key, node);

            if self.map.len() > self.capacity {
                if let Some(tail) = self.tail.take() {
                    let ListNode { key: tail_key, prev: tail_prev, .. } = &mut *tail.borrow_mut();
                    if let Some(new_tail) = tail_prev {
                        let ListNode { next: new_tail_next, .. }  = &mut *new_tail.borrow_mut();
                        *new_tail_next = None;
                        _ = self.tail.insert(Rc::clone(new_tail));
                    }
                    else {
                        self.head = None;
                        self.tail = None;
                    }
                    self.map.remove(tail_key);
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capacity_1() {
        let mut cache = LRUCache::new(1);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(-1, cache.get(1));
        cache.put(3, 3);
        assert_eq!(-1, cache.get(2));
        cache.put(4, 4);
        assert_eq!(-1, cache.get(1));
        assert_eq!(-1, cache.get(3));
        assert_eq!(4, cache.get(4));
    }

    #[test]
    fn test_capacity_2() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(1, cache.get(1));
        cache.put(3, 3);
        assert_eq!(-1, cache.get(2));
        cache.put(4, 4);
        assert_eq!(-1, cache.get(1));
        assert_eq!(3, cache.get(3));
        assert_eq!(4, cache.get(4));
    }
}