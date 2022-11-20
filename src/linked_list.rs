use std::{cell::RefCell, fmt, rc::Rc};

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

    pub fn append(&mut self, item: T) {
        let node = Rc::new(RefCell::new(Node::new(item, None, None)));

        // case 1: empty list
        if self.len == 0 {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(Rc::clone(&node));

        // case 2: one node list
        } else if self.len == 1 {
            if let Some(tail) = self.tail.clone() {
                // let mut tail = tail.borrow_mut();
                // tail.next = Some(Rc::clone(&node));
                tail.borrow_mut().next = Some(Rc::clone(&node));
                node.borrow_mut().prev = Some(Rc::clone(&tail));
            }

        // case 3: at least 2 nodes in list
        } else {
            if let Some(tail) = self.tail.clone() {
                tail.borrow_mut().next = Some(Rc::clone(&node));
                node.borrow_mut().prev = Some(Rc::clone(&tail));
            }
        }

        self.tail = Some(Rc::clone(&node));
        self.len += 1;
    }

    pub fn prepend(&mut self, _item: T) {
        todo!()
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
}
