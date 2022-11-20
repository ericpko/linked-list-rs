use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

pub struct Node<T> {
    pub item: T,
    pub next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(item: T, next: Link<T>) -> Self {
        Self { item, next }
    }
}

pub struct SinglyLinkedList<T> {
    pub head: Link<T>,
    pub len: u32,
}

impl<T> SinglyLinkedList<T> {
    pub fn new(list: Option<Vec<T>>) -> Self {
        let mut head: Link<T> = None;
        let mut len: u32 = 0;
        if let Some(items) = list {
            len = items.len() as u32;
            for item in items.into_iter().rev() {
                head = Some(Rc::new(Node::new(item, head.clone())));
            }
        }

        Self { head, len }
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
            let node = Rc::new(Node::new(item, None));
            self.head = Some(Rc::clone(&node));
            self.tail = Some(Rc::clone(&node));
        }

        self.len += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_list() {
        let arr = vec![1, 2, 3, 4, 5];
        let mut list = SinglyLinkedList::new(Some(arr.clone()));
        let mut i = 0;
        while let Some(curr) = list.head {
            println!("{}", curr.item);
            assert_eq!(arr[i], curr.item);
            i += 1;
            list.head = curr.next.clone();
        }
    }
}
