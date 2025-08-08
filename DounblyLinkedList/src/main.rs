use std::rc::{Rc, Weak};
use std::cell::RefCell;

// 節點結構
#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }))
    }
}

// 雙向鏈結串列
#[derive(Debug)]
struct DoublyLinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    size: usize,
}

impl DoublyLinkedList {
    fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }

    // 在尾端加入新節點
    fn push_back(&mut self, value: i32) {
        let new_node = Node::new(value);
        
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                self.tail = Some(new_node);
            }
            None => {
                // 第一個節點
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
        self.size += 1;
    }

    // 在頭端加入新節點
    fn push_front(&mut self, value: i32) {
        let new_node = Node::new(value);
        
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
            None => {
                // 第一個節點
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
        self.size += 1;
    }

    // 從頭開始印出所有節點
    fn print_forward(&self) {
        println!("Forward:");
        let mut current = self.head.clone();
        while let Some(node) = current {
            println!("{}", node.borrow().value);
            current = node.borrow().next.clone();
        }
    }

    // 從尾開始倒著印出所有節點
    fn print_backward(&self) {
        println!("Backward:");
        let mut current = self.tail.clone();
        while let Some(node) = current {
            println!("{}", node.borrow().value);
            current = node.borrow().prev.as_ref().and_then(|weak| weak.upgrade());
        }
    }

    // 取得串列大小
    fn len(&self) -> usize {
        self.size
    }

    // 檢查是否為空
    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

fn main() {
    let mut dll = DoublyLinkedList::new();
    dll.push_back(1);
    dll.push_back(2);
    dll.push_back(3);
    dll.push_back(4);

    dll.print_forward();
    dll.print_backward();

    dll.push_front(1);
    dll.push_front(2);
    dll.push_front(3);
    dll.push_front(4);



    dll.print_forward();
    dll.print_backward();



}