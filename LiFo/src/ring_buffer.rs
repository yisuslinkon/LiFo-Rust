#[derive(Debug, PartialEq)]
pub struct RingBuffer<T> {
    front: i16,
    rear: i16,
    capacity: usize,
    items: Vec<Option<T>>
}

pub struct  Iter<'a, T> {
    ring: &'a RingBuffer<T>,
    position: usize
}

impl<T> RingBuffer<T> {
    pub fn new(size: usize) -> Self {
        if size == 0 {
            panic!("Use a size bigger than 0");
        }
        RingBuffer { 
            front: -1, 
            rear: -1,
            capacity: size, 
            items: Vec::with_capacity(size)
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { ring: self, position: 0}
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        let len = self.len();
        if len == 0 {
            let (head, tail) = self.items.split_at_mut(0);
            return IterMut { inner: head.iter_mut().chain(tail.iter_mut()) };
        }

        let start = self.front as usize;
        let first_len = (self.items.len() - start).min(len);
        let wrapped = len - first_len;

        let (head, tail) = self.items.split_at_mut(start);
        IterMut {
            inner: tail[..first_len].iter_mut().chain(head[..wrapped].iter_mut()),
        }
    }

    pub fn len(&self) -> usize {
        if self.front == -1 {
            return 0
        } else if self.rear >= self.front  {
            return (self.rear - self.front + 1) as usize
        }
        self.capacity - self.front as usize + self.rear as usize + 1
    }

    pub fn is_full(&self) -> bool {
        (self.rear + 1) % self.capacity as i16 == self.front
    }

    pub fn is_empty(&self) -> bool {
        self.front == -1
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None
        }
        Some(self.items[self.front as usize].as_ref().unwrap())
    }

    pub fn tail(&self) -> Option<&T> {
        if self.is_empty() {
            return None
        }
        Some(self.items[self.rear as usize].as_ref().unwrap())
    }


    pub fn enqueue(&mut self, element: T) -> Option<&mut Self> {
        if self.is_full() {
            return  None;
        }

        if self.is_empty() {
            self.front = 0;
            self.rear = 0;
            self.items.push(Some(element));
            return Some(self)
        } 
        self.rear = (self.rear + 1) % self.capacity as i16;

        if self.items.len() <= self.rear as usize {
            self.items.push(Some(element));
        } else {
            self.items[self.rear as usize] = Some(element);
        }

        Some(self)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None
        }

        let front_value: Option<T> = self.items[self.front as usize].take();

        if self.front == self.rear {
            self.front = -1;
            self.rear = -1;
        } else {
            self.front = (self.front + 1) % self.capacity as i16;
        }

        front_value
    }
}


impl<T> Default for RingBuffer<T> {
    fn default() -> Self {
        RingBuffer::<T>::new(1)
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.ring.len() {
            return None;
        }
        let idx = (self.ring.front as usize + self.position) % self.ring.capacity;
        self.position += 1;
        Some(
            self.ring.items[idx].as_ref().expect("slot inside occupied range was empty")
        )
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.ring.len() - self.position;
        (remaining, Some(remaining))
    }

}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {}

impl<'a, T> IntoIterator for &'a RingBuffer<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct IterMut<'a, T> {
    inner: std::iter::Chain<
        std::slice::IterMut<'a, Option<T>>,
        std::slice::IterMut<'a, Option<T>>,
    >,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let slot = self.inner.next()?;
        Some(slot.as_mut().expect("slot inside occupied run was empty"))
    }
}

impl<'a, T> IntoIterator for &'a mut RingBuffer<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub struct IntoIter<T> {
    ring: RingBuffer<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ring.dequeue()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.ring.len();
        (remaining, Some(remaining))
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

impl<T> IntoIterator for RingBuffer<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { ring: self }
    }
}

#[cfg(test)] 
mod test {
    use super::*;

    #[test]
    #[should_panic(expected = "Use a size bigger than 0")]
    fn test_init() {
        let queue: RingBuffer<u32> = RingBuffer::new(0);
        assert_eq!(queue.front, -1);
        assert_eq!(queue.rear, -1);
        assert_eq!(queue.capacity, 0);
        assert_eq!(queue.items.len(), 0);
    }

    #[test]
    fn test_enqueue_empty() {
        let q: RingBuffer<u32> = RingBuffer::new(7);
        assert!(q.is_empty());
        assert_eq!(q.peek(), None);
        assert_eq!(q.tail(), None);
    }

    #[test]
    fn test_enqueue_not_empty() {
        let mut q: RingBuffer<u32> = RingBuffer::new(7);
        q.enqueue(0);
        assert!(!q.is_empty());
    }

    #[test]
    fn test_enqueue_full() {
        let mut q: RingBuffer<u32> = RingBuffer::new(3);
        q.enqueue(0);
        q.enqueue(1);
        q.enqueue(2);
        assert!(q.is_full());
        assert_eq!(q.enqueue(3), None);
    }

    #[test]
    fn test_queue_peek_without_dequeue() {
        let mut q: RingBuffer<u32> = RingBuffer::new(3);
        q.enqueue(0);
        q.enqueue(1);
        q.enqueue(2);
        assert_eq!(q.peek(), Some(&0));
    }

    #[test]
    fn test_queue_peek_with_dequeue() {
        let mut q: RingBuffer<u32> = RingBuffer::new(3);
        q.enqueue(0);
        q.enqueue(1);
        q.enqueue(2);
        let dequeue_item: Option<u32> = q.dequeue();
        assert_eq!(dequeue_item, Some(0));
        assert_eq!(q.peek(), Some(&1));
    }

    #[test]
    fn test_queue_full_round() {
        let mut q: RingBuffer<u32> = RingBuffer::new(6);
        q.enqueue(5);
        q.enqueue(10);
        q.enqueue(15);
        q.enqueue(20);
        q.enqueue(25);
        q.enqueue(30);
        assert_eq!(q.enqueue(35), None);
        q.dequeue();
        q.dequeue();
        assert_eq!(q.peek(), Some(&15));
        q.enqueue(35);
        q.enqueue(40);
        assert_eq!(q.tail(), Some(&40));
    }

    #[test]
    fn test_iterators() {
        let mut q: RingBuffer<u32> = RingBuffer::new(6);
        q.enqueue(5);
        q.enqueue(10);
        q.enqueue(15);
        q.enqueue(20);
        q.enqueue(25);
        q.enqueue(30);

        let borrowed: Vec<&u32> = q.iter().collect();
        assert_eq!(borrowed, vec![&5, &10, &15, &20, &25, &30]);

        for x   in &mut q {
            *x -= 5;
        }

        let owned: Vec<u32> = q.into_iter().collect();
        assert_eq!(owned, vec![0, 5, 10, 15, 20, 25]);
    }

    #[test]
    fn drain_then_reuse() {
        let mut q: RingBuffer<u32> = RingBuffer::new(3);
        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);
        assert_eq!(q.dequeue(), Some(1));
        assert_eq!(q.dequeue(), Some(2));
        assert_eq!(q.dequeue(), Some(3));
        assert!(q.is_empty());
        assert_eq!(q.peek(), None);
    }

    #[test]
    fn iterates_across_the_wrap() {
        let mut q: RingBuffer<u32> = RingBuffer::new(4);
        q.enqueue(1); q.enqueue(2); q.enqueue(3);
        q.dequeue(); q.dequeue();
        q.enqueue(4); q.enqueue(5);
        let got: Vec<&u32> = q.iter().collect();
        assert_eq!(got, vec![&3, &4, &5]);
    }

    #[test]
    fn iter_mut_partial_and_wrapped() {
        let mut q: RingBuffer<u32> = RingBuffer::new(4);
        q.enqueue(1); q.enqueue(2); q.enqueue(3);
        q.dequeue();
        for x in &mut q { *x *= 10; }
        let got: Vec<&u32> = q.iter().collect();
        assert_eq!(got, vec![&20, &30]);

        q.enqueue(4); q.enqueue(5);
        for x in &mut q { *x += 1; }
        let got: Vec<&u32> = q.iter().collect();
        assert_eq!(got, vec![&21, &31, &5, &6]);
    }

    #[test]
    fn into_iter_partial() {
        let mut q: RingBuffer<u32> = RingBuffer::new(4);
        q.enqueue(1); q.enqueue(2); q.enqueue(3);
        q.dequeue();
        let owned: Vec<u32> = q.into_iter().collect();
        assert_eq!(owned, vec![2, 3]);
    }
}