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

// Iterators implementation
pub struct IntoIter<T>(StackLinked<T>);

impl<T> StackLinked<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}


pub struct Iter<'a, T> {
    next: Option<&'a Box<Node<T>>>,
}

impl<T> StackLinked<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref();
            &node.val
        })
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

    #[test]
    fn push_pop_multi() {
        let mut s = StackLinked::new();
        for i in 0..100 {
            s.push(i);
        }
        for i in 0..100 {
            assert_eq!(s.pop(), Some(100-i-1));
        }
    }

    #[test]
    fn into_iter() {
        let mut s = StackLinked::new();
        s.push(1);
        s.push(2);
        s.push(3);

        let mut iter = s.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn into_iter_loop() {
        let mut s = StackLinked::new();
        s.push(0);
        s.push(1);
        s.push(2);

        for (i, v) in s.into_iter().enumerate() {
            assert_eq!(v, 2-i);
        }
    }

    #[test]
    fn iter() {
        let mut s = StackLinked::new();
        s.push(1);
        s.push(2);
        s.push(3);

        let mut iter = s.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ter_loop() {
        let mut s = StackLinked::new();
        s.push(0);
        s.push(1);
        s.push(2);

        for (i, v) in s.iter().enumerate() {
            assert_eq!(v, &(2-i));
        }
    }
}