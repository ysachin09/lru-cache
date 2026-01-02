use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node<K, V> {
    key: K,
    value: V,
    prev: Option<Rc<RefCell<Node<K, V>>>>,
    next: Option<Rc<RefCell<Node<K, V>>>>,
}

pub struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, Rc<RefCell<Node<K, V>>>>,
    head: Option<Rc<RefCell<Node<K, V>>>>,
    tail: Option<Rc<RefCell<Node<K, V>>>>,
}

impl<K: std::hash::Hash + Eq + Clone, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        LruCache {
            capacity,
            map: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<V> where V: Clone, {
        let node = self.map.get(key)?.clone();
        self.detach(node.clone());
        self.attach_to_head(node.clone());
        Some(node.borrow().value.clone())
    }

    fn detach(&mut self, node: Rc<RefCell<Node<K, V>>>) {
        let prev = node.borrow().prev.clone();
        let next = node.borrow().next.clone();
    
        match prev.clone() {
            Some(p) => {
                p.borrow_mut().next = next.clone();
            }
            None => {
                // node is head
                self.head = next.clone();
            }
        }
    
        match next.clone() {
            Some(n) => {
                n.borrow_mut().prev = prev.clone();
            }
            None => {
                // node is tail
                self.tail = prev.clone();
            }
        }
    
        node.borrow_mut().prev = None;
        node.borrow_mut().next = None;
    }
    

    fn attach_to_head(&mut self, node: Rc<RefCell<Node<K, V>>>) {
        let current_head = self.head.clone();
        node.borrow_mut().next = current_head.clone();
        node.borrow_mut().prev = None;
        if let Some(current_head) = current_head {
            current_head.borrow_mut().prev = Some(node.clone());
        }
        self.head = Some(node.clone());
        if self.tail.is_none() {
            self.tail = Some(node.clone());
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if let Some(node) = self.map.get(&key).cloned() {
            node.borrow_mut().value = value;
            self.detach(node.clone());
            self.attach_to_head(node);
            return;
        }
    
        let node = Rc::new(RefCell::new(Node {
            key,
            value,
            prev: None,
            next: None,
        }));
    
        let key_ref = node.borrow().key.clone();
        self.map.insert(key_ref, node.clone());
        self.attach_to_head(node.clone());
    
        if self.map.len() > self.capacity {
            if let Some(tail) = self.tail.clone() {
                let evicted_key = tail.borrow().key.clone();
                self.detach(tail);
                self.map.remove(&evicted_key);
            }
        }
    }
    
}
