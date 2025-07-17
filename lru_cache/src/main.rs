use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::{Rc, Weak};

/// A node in the doubly-linked list
struct Node<K, V> {
    key: K,
    value: V,
    prev: Option<Weak<RefCell<Node<K, V>>>>,
    next: Option<Rc<RefCell<Node<K, V>>>>,
}

/// LRU Cache implementation
pub struct LRUCache<K, V> {
    capacity: usize,
    map: HashMap<K, Rc<RefCell<Node<K, V>>>>,
    head: Option<Rc<RefCell<Node<K, V>>>>,
    tail: Option<Rc<RefCell<Node<K, V>>>>,
}

impl<K, V> LRUCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Create a new LRUCache with given capacity
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be at least 1");
        LRUCache {
            capacity,
            map: HashMap::with_capacity(capacity),
            head: None,
            tail: None,
        }
    }

    /// Get a value from the cache, updating its recency
    pub fn get(&mut self, key: &K) -> Option<V> {
        if let Some(node_rc) = self.map.get(key) {
            // Move this node to front
            let value = node_rc.borrow().value.clone();
            self.detach(node_rc);
            self.attach_front(node_rc.clone());
            Some(value)
        } else {
            None
        }
    }

    /// Insert or update a value in the cache
    pub fn put(&mut self, key: K, value: V) {
        if let Some(node_rc) = self.map.get(&key) {
            // Update existing
            node_rc.borrow_mut().value = value;
            self.detach(node_rc);
            self.attach_front(node_rc.clone());
        } else {
            // Insert new
            if self.map.len() == self.capacity {
                // Evict least recently used
                self.evict_lru();
            }
            let new_node = Rc::new(RefCell::new(Node {
                key: key.clone(),
                value,
                prev: None,
                next: None,
            }));
            self.attach_front(new_node.clone());
            self.map.insert(key, new_node);
        }
    }

    /// Detach a node from its current position
    fn detach(&mut self, node: &Rc<RefCell<Node<K, V>>>) {
        let mut node_borrow = node.borrow_mut();

        // Update prev's next
        if let Some(prev_weak) = node_borrow.prev.take() {
            if let Some(prev_rc) = prev_weak.upgrade() {
                prev_rc.borrow_mut().next = node_borrow.next.clone();
            }
        } else {
            // Was head
            self.head = node_borrow.next.clone();
        }

        // Update next's prev
        if let Some(next_rc) = node_borrow.next.take() {
            next_rc.borrow_mut().prev = node_borrow.prev.clone();
        } else {
            // Was tail
            self.tail = node_borrow.prev.as_ref().and_then(|weak| weak.upgrade());
        }
    }

    /// Attach a node to the front (head) of the list
    fn attach_front(&mut self, node: Rc<RefCell<Node<K, V>>>) {
        node.borrow_mut().next = self.head.clone();
        node.borrow_mut().prev = None;

        if let Some(old_head) = &self.head {
            old_head.borrow_mut().prev = Some(Rc::downgrade(&node));
        }
        self.head = Some(node.clone());

        if self.tail.is_none() {
            // First node
            self.tail = Some(node);
        }
    }

    /// Evict the least recently used (tail) node
    fn evict_lru(&mut self) {
        if let Some(old_tail) = self.tail.take() {
            let key_to_remove = old_tail.borrow().key.clone();
            self.detach(&old_tail);
            self.map.remove(&key_to_remove);
        }
    }

    /// Current size of cache
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Whether the cache is empty
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::LRUCache;

    #[test]
    fn test_lru_cache() {
        let mut cache = LRUCache::new(2);
        cache.put(1, "one");
        cache.put(2, "two");
        assert_eq!(cache.get(&1), Some("one"));
        cache.put(3, "three");
        // 2 should be evicted
        assert_eq!(cache.get(&2), None);
        cache.put(4, "four");
        // 1 should be evicted
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&3), Some("three"));
        assert_eq!(cache.get(&4), Some("four"));
    }
}
