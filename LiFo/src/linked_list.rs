#[derive(Debug, Clone, PartialEq)]
pub struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            item: value,
            next: None
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LinkList<T> {
    head: Option<Box<Node<T>>>
}

impl<T> Default for LinkList<T> {
    fn default() -> Self {
        LinkList::<T>::new()
    }
}

impl<T> LinkList<T> {
    pub fn new() -> Self {
        LinkList {
            head: None
        }
    }

    pub fn push(&mut self, value: T) {
        let mut node: Node<T> = Node::new(value);
        node.next = self.head.take();
        self.head = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(old) => {
                self.head = old.next;
                Some(old.item)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.item)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.item)
    }
}

impl<T> Drop for LinkList<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_test() {
        let l: LinkList<u32> = LinkList::new();
        assert_eq!(l.head, None);
    }

    #[test]
    fn test_simple_push() {
        let mut l: LinkList<u32> = LinkList::new();
        l.push(1);
        l.push(2);
        l.push(3);
        assert_ne!(l.head, None);
        assert_eq!(l.head.as_ref().unwrap().item, 3);
        assert_eq!(l.head.as_ref().unwrap().next.as_ref().unwrap().item, 2);
        assert_eq!(l.head.as_ref().unwrap().next.as_ref().unwrap().next.as_ref().unwrap().item, 1);
    }

    #[test]
    fn test_empty_pop() {
        let mut l: LinkList<u32> = LinkList::new();
        assert_eq!(l.pop(), None);
    }

    #[test]
    fn test_empty_peek() {
        let l: LinkList<u32> = LinkList::new();
        assert_eq!(l.peek(), None);
    }

    #[test]
    fn test_simple_pop() {
        let mut l: LinkList<u32> = LinkList::new();
        l.push(1);
        l.push(2);
        l.push(3);
        l.pop();
        l.pop();
        l.push(4);
        l.push(5);
        l.pop();
        l.push(6);
        assert_ne!(l.head, None);
        assert_eq!(l.head.as_ref().unwrap().item, 6);
        assert_eq!(l.head.as_ref().unwrap().next.as_ref().unwrap().item, 4);
        assert_eq!(l.head.as_ref().unwrap().next.as_ref().unwrap().next.as_ref().unwrap().item, 1);
    }

    #[test]
    fn test_simple_pop_api() {
        let mut l: LinkList<u32> = LinkList::new();
        l.push(1);
        l.push(2);
        l.push(3);
        assert_eq!(l.pop(), Some(3));
        l.push(4);
        assert_eq!(l.pop(), Some(4));
        l.push(5);
        assert_eq!(l.pop(), Some(5));
        l.push(6);
        assert_eq!(l.pop(), Some(6));
        assert_eq!(l.pop(), Some(2));
        assert_eq!(l.pop(), Some(1));
        assert_eq!(l.pop(), None);
    }

    #[test]
    fn test_simple_peek() {
        let mut l: LinkList<u32> = LinkList::new();
        l.push(1);
        l.push(2);
        l.push(3);
        l.pop();
        l.pop();
        l.push(4);
        l.push(5);
        l.pop();
        l.push(6);
        assert_eq!(l.peek(), Some(&6));
    }

    #[test]
    fn test_simple_peek_mut() {
        let mut l: LinkList<u32> = LinkList::new();
        l.push(1);
        l.push(2);
        l.push(3);
        l.pop();
        l.pop();
        l.push(4);
        l.push(5);
        l.pop();
        l.push(6);

        if let Some(x) = l.peek_mut() {
            *x += 10;
        }

        assert_eq!(l.peek(), Some(&16));
    }

    #[test]
    fn test_drop_success() {
        // Justo to validate avoid memmory issues stack overflow
        let mut l: LinkList<u32> = LinkList::new();

        for x in 0..100_000 {
            l.push(x)
        }
    }
}