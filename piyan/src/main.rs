// use rand::{Rng};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct Node {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    value: i32,
}

struct DoubleyLinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    size: u32,
}

impl Node {
    fn new(val: i32) -> Node {
        Node {
            head: None,
            tail: None,
            value: val,
        }
    }
}
impl DoubleyLinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    fn push_back(&mut self, value: i32) {
        let raw_node = Node::new(value);
        let new_node = Rc::new(RefCell::new(raw_node));
        match &self.tail {
            Some(old_node) => {
                old_node.borrow_mut().tail = Some(new_node.clone());
                new_node.borrow_mut().head = Some(old_node.clone());
                self.tail = Some(new_node);
            }, 
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }

    }
    fn push_front(&mut self, value: i32) {
        let raw_node = Node::new(value);
        let new_node=  Rc::new(RefCell::new(raw_node));
        match &self.head {
            Some(old_head) => {
                old_head.borrow_mut().head = Some(new_node.clone());
                new_node.borrow_mut().tail = Some(old_head.clone());
                self.head = Some(new_node);
            },
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }
    fn pop_back(&mut self) {
        match self.tail.take() {
            Some(old_tail) => {
                match old_tail.borrow_mut().head.take() {
                    Some(prev_node) => {
                        prev_node.borrow_mut().tail = None;
                        self.tail = Some(prev_node);

                    },
                    None => {
                        self.head = None;
                        self.tail = None;
                    }
                }
            },
            None => {
                println!("no tail");
            }
        }
    }
    fn print_backward(&self) {

        let mut current = self.tail.clone();
        while let Some(now) = current {
            print!("{} ", now.borrow().value);
            current = now.borrow().head.clone();
        }
        println!();
    }
    fn print_forward(&self) {
        let mut current = self.head.clone();
        while let Some(now) = current {
            print!("{} " , now.borrow().value);
            current = now.borrow().tail.clone();
        }
        println!();
    }
}

fn main() {
    let mut dll: DoubleyLinkedList = DoubleyLinkedList::new();
    dll.push_back(3);
    dll.push_back(22);
    dll.push_back(3330);
    dll.push_back(1);
    
    dll.print_forward();
    dll.print_backward();
    dll.pop_back();
    dll.print_forward();
    dll.pop_back();
    dll.pop_back();
    dll.print_forward();
    dll.pop_back();
    dll.pop_back();
    dll.pop_back();
    dll.pop_back();

    dll.print_forward();


    dll.push_front(1000);
    dll.push_front(122);

    dll.push_front(10);

    dll.push_front(3);

    dll.print_forward();
    
}
