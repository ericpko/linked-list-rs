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

pub struct LinkedList<T> {
    pub head: Link<T>,
    pub size: u32,
}

impl<T> LinkedList<T> {
    pub fn new(list: Option<Vec<T>>) -> Self {
        let mut head: Link<T> = None;
        let mut size: u32 = 0;
        if let Some(items) = list {
            size = items.len() as u32;
            for item in items.into_iter().rev() {
                head = Some(Rc::new(Node::new(item, head.clone())));
            }
        }

        Self { head, size }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_list() {
        let list = vec![1, 2, 3, 4, 5];
        let mut list = LinkedList::new(Some(list));
        while let Some(curr) = list.head {
            println!("{}", curr.item);
            list.head = curr.next.clone();
        }
    }
}
