use std::collections::HashMap;

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, (i32, Node)>,
    head: Option<Box<Node>>,
    tail: *mut Node,
}

struct Node {
    key: i32,
    value: i32,
    prev: *mut Node,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(key: i32, value: i32) -> Self {
        Node {
            key,
            value,
            prev: std::ptr::null_mut(),
            next: None,
        }
    }
}

impl LRUCache {
    fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            map: HashMap::new(),
            head: None,
            tail: std::ptr::null_mut(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some((value, node)) = self.map.get_mut(&key) {
            self.move_to_front(&mut *node);
            *value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some((_, node)) = self.map.get_mut(&key) {
            node.value = value;
            self.move_to_front(&mut *node);
        } else {
            if self.map.len() >= self.capacity {
                self.evict_tail();
            }

            let new_node = Box::new(Node::new(key, value));
            self.add_to_front(new_node);
        }
    }

    fn move_to_front(&mut self, node: &mut Node) {
        if self.head.is_some() && &**self.head.as_ref().unwrap() as *const Node != node {
            // Unlink the node from the current position
            let prev = node.prev;
            let next = node.next.take();

            if !prev.is_null() {
                unsafe { (*prev).next = next; }
            } else {
                self.head = next;
            }

            if let Some(mut n) = next {
                n.prev = prev;
            } else {
                self.tail = prev;
            }

            // Move the node to the front
            node.next = self.head.take();
            node.prev = std::ptr::null_mut();

            if let Some(n) = &mut node.next {
                n.prev = node;
            } else {
                self.tail = &mut *node;
            }

            self.head = Some(node);
        }
    }

    fn add_to_front(&mut self, node: Box<Node>) {
        let raw_node: *mut Node = &mut *node;
        if let Some(mut head) = self.head.take() {
            head.prev = raw_node;
            node.next = Some(head);
            self.head = Some(node);
        } else {
            self.head = Some(node);
            self.tail = raw_node;
        }
        self.map.insert(node.key, (node.value, *node));
    }

    fn evict_tail(&mut self) {
        if let Some(tail) = unsafe { self.tail.as_mut() } {
            self.map.remove(&tail.key);
            if !tail.prev.is_null() {
                unsafe { (*tail.prev).next = None; }
                self.tail = tail.prev;
            } else {
                self.head = None;
                self.tail = std::ptr::null_mut();
            }
        }
    }
}
