use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

pub struct Node<T> {
    pub item: T,
    pub next: Link<T>,
    pub prev: Link<T>,
}

impl<T> Node<T> {
    pub fn new(item: T, next: Link<T>, prev: Link<T>) -> Self {
        Self { item, next, prev }
    }
}

pub struct LinkedList<T> {
    pub head: Link<T>,
    pub tail: Link<T>,
    len: u32,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn append(&mut self, item: T) {
        // case 1: empty list
        if self.len == 0 {
            let node = Rc::new(Node::new(item, None, None));
            self.head = Some(Rc::clone(&node));
            self.tail = Some(Rc::clone(&node));
        }

        self.len += 1;
    }
}
