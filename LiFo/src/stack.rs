/// The iteration order with satisfy the order of LIFO algorithm
/// The Last element that enter on the stack, will be the first one to display.
#[derive(Debug)]
pub struct Stack<T> {
    items: Vec<T>
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::<T>::new()
        }
    }

    pub fn push(&mut self, element: T) {
        self.items.push(element)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}


impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = std::iter::Rev<std::vec::IntoIter<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter().rev()
    }
}

impl<'a, T> IntoIterator for &'a Stack<T> {
    type Item = &'a T;
    type IntoIter = std::iter::Rev<std::slice::Iter<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter().rev()
    }
}

impl <'a, T> IntoIterator for &'a mut Stack<T> {
    type Item = &'a mut T;
    type IntoIter = std::iter::Rev<std::slice::IterMut<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut().rev()
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_stack() {
        let stack_empty: Stack<u16> = Stack::new();
        assert!(stack_empty.is_empty());

        let mut stack_with_number: Stack<u16> = Stack::new();
        stack_with_number.push(1);
        assert!(!stack_with_number.is_empty());
    }

    #[test]
    fn test_push() {
        let mut stack_empty: Stack<u16> = Stack::new();
        stack_empty.push(15);
        stack_empty.push(12);
        stack_empty.push(16);

        assert!(!stack_empty.is_empty());
        assert_eq!(stack_empty.peek(), Some(&16));
    }

    #[test]
    fn test_pop() {
        let mut stack_with_numbers: Stack<u16> = Stack::new();
        stack_with_numbers.push(1);
        stack_with_numbers.push(2);
        stack_with_numbers.push(3);
        stack_with_numbers.push(4);
        stack_with_numbers.push(5);
        let last_number: u16 = stack_with_numbers.pop().unwrap();

        assert_eq!(*stack_with_numbers.peek().unwrap(), 4);
        assert_eq!(last_number, 5);
    }

    #[test]
    fn test_lifo() {
        let mut stack_test: Stack<f64> = Stack::new();

        stack_test.push(0.0);
        stack_test.push(0.1);
        stack_test.push(0.2);
        stack_test.push(0.3);

        assert_eq!(stack_test.pop(), Some(0.3));
        assert_eq!(stack_test.pop(), Some(0.2));
        assert_eq!(stack_test.pop(), Some(0.1));
        assert_eq!(stack_test.peek(), Some(&0.0));

    }

    #[test]
    fn test_empty_peek() {
        let stack_empty: Stack<u16> = Stack::new();
        assert_eq!(stack_empty.peek(), None);
    }

    #[test]
    fn test_empty_pop() {
        let mut stack_empty: Stack<u16> = Stack::new();
        assert_eq!(stack_empty.pop(), None);
    }

    #[test]
    fn test_iter() {
        let mut s: Stack<i32> = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);
        s.push(4);
        s.push(5);

        let borrowed: Vec<&i32> = (&s).into_iter().collect();
        assert_eq!(borrowed, vec![&5, &4, &3, &2, &1]);

        for n in &mut s {
            *n += 1;
        }

        let owned: Vec<i32> = s.into_iter().collect();
        assert_eq!(owned, vec![6, 5, 4, 3, 2]);
    }
}