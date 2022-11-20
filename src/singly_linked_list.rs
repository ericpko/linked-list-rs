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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_list() {
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
