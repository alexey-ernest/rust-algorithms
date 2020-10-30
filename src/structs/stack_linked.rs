/// A stack implemented as a linked-list.
pub struct StackLinked<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
}

impl<T> StackLinked<T> {
    pub fn new() -> Self {
        StackLinked {
            head: Link::None,
        }
    }

    pub fn push(&mut self, val: T) {
        let node = Box::new(Node {
            val,
            next: self.head.take(),
        });
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|mut boxed| {
            self.head = boxed.next.take();
            boxed.val
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|boxed| {
            &boxed.val
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|boxed| {
            &mut boxed.val
        })
    }
}

impl<T> Drop for StackLinked<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut boxed) = link {
            link = boxed.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_test() {
        let s = StackLinked::<i32>::new();
        assert_eq!(s.peek(), None);
    }

    #[test]
    fn push_peek_pop() {
        let mut s = StackLinked::new();
        assert_eq!(s.peek(), None);
        s.push(1);
        assert_eq!(s.peek(), Some(&1));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.peek(), None);
    }

    #[test]
    fn peek_mut() {
        let mut s = StackLinked::new();
        s.push(1);
        assert_eq!(s.peek(), Some(&1));
        s.peek_mut().map(|val| {
            *val = 2;
        });
        assert_eq!(s.peek(), Some(&2));
    }
}