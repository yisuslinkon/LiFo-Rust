#[derive(Debug, PartialEq)]
pub struct RingBuffer<T> {
    front: i16,
    rear: i16,
    capacity: usize,
    items: Vec<Option<T>>
}

impl<T: Clone> RingBuffer<T> {
    pub fn new(size: usize) -> Self {
        RingBuffer { 
            front: -1, 
            rear: -1,
            capacity: size, 
            items: Vec::with_capacity(size)
        }
    }

    pub fn is_full(&self) -> bool {
        (self.rear + 1) % self.capacity as i16 == self.front
    }

    pub fn is_empty(&self) -> bool {
        self.front == -1
    }

    pub fn enqueue(&mut self, element: T) -> Option<&mut Self> {
        if self.is_full() {
            println!("Currently the queue is full");
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
            println!("The queue is empty, you can't drop it any element");
            return None
        }

        if self.items.len() == 1 {
            self.front = -1;
            self.rear = -1;
            return self.items.pop().unwrap()
        }
        let front_value: T = self.items[self.front as usize].clone().unwrap();
        self.items[self.front as usize] = None;
        self.front += 1;

        Some(front_value)

    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            println!("The queue is empty");
            return None
        }
        Some(self.items[self.front as usize].as_ref().unwrap())
    }

    pub fn tail(&self) -> Option<&T> {
        if self.is_empty() {
            println!("The queue is empty");
            return None
        }
        Some(self.items[self.rear as usize].as_ref().unwrap())
    }
}

impl<T: Clone> Default for RingBuffer<T> {
    fn default() -> Self {
        RingBuffer::<T>::new(0)
    }
}


#[cfg(test)] 
mod test {
    use super::*;

    #[test]
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
}