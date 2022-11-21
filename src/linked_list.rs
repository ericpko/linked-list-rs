use std::{
    cell::{Ref, RefCell, RefMut},
    fmt,
    rc::Rc,
};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

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

    pub fn push_front(&mut self, item: T) {
        let new_head = Rc::new(RefCell::new(Node::new(item, None, None)));
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::clone(&new_head));
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.head = Some(Rc::clone(&new_head));
                self.tail = Some(new_head);
            }
        }
        self.len += 1;
    }

    pub fn push_back(&mut self, item: T) {
        let new_tail = Rc::new(RefCell::new(Node::new(item, None, None)));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&new_tail));
                new_tail.borrow_mut().prev = Some(old_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }

        self.len += 1;
    }

    pub fn append(&mut self, item: T) {
        let node = Rc::new(RefCell::new(Node::new(item, None, None)));

        // case 1: empty list
        if self.len == 0 {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(Rc::clone(&node));

        // case 2: at least one item in list
        } else {
            if let Some(tail) = self.tail.clone() {
                tail.borrow_mut().next = Some(Rc::clone(&node));
                node.borrow_mut().prev = Some(Rc::clone(&tail));
            }
        }

        self.tail = Some(Rc::clone(&node));
        self.len += 1;
    }

    pub fn prepend(&mut self, item: T) {
        let new_head = Rc::new(RefCell::new(Node::new(item, None, None)));
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::clone(&new_head));
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.head = Some(Rc::clone(&new_head));
                self.tail = Some(new_head);
            }
        }
        self.len += 1;
    }

    pub fn _pop(&mut self) -> Option<T>
    where
        T: Copy,
    {
        let mut val = None;

        // case 1: empty list
        if self.len == 0 {
            return val;

        // case 2: one item in list
        } else if self.len == 1 {
            if let Some(tail) = self.tail.clone() {
                val = Some(tail.borrow().item);
                self.head = None;
                self.tail = None;
            }

        // case 3: at least two items in list
        } else {
            if let Some(tail) = self.tail.clone() {
                val = Some(tail.borrow().item);
                if let Some(new_tail) = tail.borrow_mut().prev.clone() {
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail.clone());
                }
            }
        }

        self.len -= 1;
        val
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut val = None;

        // case 1: empty list
        if self.len == 0 {
            return val;

        // case 2: one item in list
        } else if self.len == 1 {
            if let Some(tail) = self.tail.take() {
                self.head = None;
                self.tail = None;
                val = Some(Rc::try_unwrap(tail).ok().unwrap().into_inner().item);
            }

        // case 3: at least two items in list
        } else {
            if let Some(old_tail) = self.tail.take() {
                if let Some(new_tail) = old_tail.borrow_mut().prev.take() {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                val = Some(Rc::try_unwrap(old_tail).ok().unwrap().into_inner().item);
            }
        }

        self.len -= 1;
        val
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.len > 0 {
            self.len -= 1;
        }

        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().item
        })
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len > 0 {
            self.len -= 1;
        }

        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().item
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        // self.head
        //     .as_ref()
        //     .map(|node| Ref::map(node.borrow(), |node| &node.item))
        if let Some(ref head) = self.head {
            let ptr = Ref::map(head.borrow(), |node| &node.item);
            return Some(ptr);
        }
        None
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|tail| Ref::map(tail.borrow(), |node| &node.item))
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        if let Some(ref head) = self.head {
            let ref_mut = RefMut::map(head.borrow_mut(), |node| &mut node.item);
            return Some(ref_mut);
        }
        None
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|tail| RefMut::map(tail.borrow_mut(), |node| &mut node.item))
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(LinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> fmt::Display for LinkedList<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut curr = self.head.clone();
        while let Some(curr_ptr) = curr {
            write!(f, "{} -> ", curr_ptr.borrow().item)?;
            curr = curr_ptr.borrow().next.clone();
        }

        writeln!(f, "None")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_append() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);
        println!("{}", list);
        println!("{}", list);
    }

    #[test]
    fn test_len() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);
        assert_eq!(5, list.len());
    }

    #[test]
    fn test_pop() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);
        let five = list.pop().unwrap();
        assert_eq!(5, five);
        assert_eq!(4, list.pop().unwrap());
        println!("{}", list);
        assert_eq!(3, list.len());
    }
}
