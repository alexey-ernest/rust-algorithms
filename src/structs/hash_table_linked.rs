use std::mem;

/// Hash table linked list implementation.
pub struct HashTableLinked<T> {
    data: Vec<Option<Box<Node<T>>>>,
}

// Node of a linked list containing key, value and a link to the next node for collided keys.
struct Node<T> {
    key: usize,
    value: T,
    next: Link<T>,
}

// Zero-abstraction enum. 
// It utilizes null-pointer layout optimization for enums (no flag bits and no waste of space)
enum Link<T> {
    Nill,
    Next(Box<Node<T>>),
}

const INIT_SIZE: usize = 997;

impl<T> HashTableLinked<T> {
    pub fn new() -> Self {
        let mut vec = Vec::with_capacity(INIT_SIZE);
        vec.resize_with(INIT_SIZE, || None::<Box<Node<T>>>);

        HashTableLinked {
            data: vec,
        }
    }

    fn get_index(&self, key: usize) -> usize {
        key % self.data.len()
    }

    pub fn set(&mut self, key: usize, value: T) {
        let index = self.get_index(key);
        match &mut self.data[index] {
            None => {
                let newnode = Box::new(Node {
                    key,
                    value,
                    next: Link::Nill,
                });
                self.data[index] = Some(newnode);
            }
            Some(ref mut b) => {
                let mut x: &mut Box<Node<T>> = b;
                while x.key != key {
                    match x.next {
                        Link::Nill => break,
                        Link::Next(ref mut n) => {
                            x = n;
                        }
                    }
                }
                if x.key == key {
                    x.value = value;
                    return;
                }

                let next = self.data[index].take().unwrap();
                let head = Box::new(Node {
                    key,
                    value,
                    next: Link::Next(next),
                });
                self.data[index] = Some(head);
            }
        }
    }

    pub fn get(&self, key: usize) -> Option<&T> {
        let index = self.get_index(key);
        match &self.data[index] {
            None => None,
            Some(ref b) => {
                let mut x = b;
                while x.key != key {
                    match x.next {
                        Link::Nill => break,
                        Link::Next(ref n) => {
                            x = n;
                        }
                    }
                }
                
                if x.key == key {
                    return Some(&x.value);
                }
                return None;
            }
        }
    }

    pub fn delete(&mut self, key: usize) {
        let index = self.get_index(key);
        match &mut self.data[index] {
            None => {},
            Some(ref mut x) => {
                if x.key == key {
                    // remove from the head
                    let next = mem::replace(&mut x.next, Link::Nill);
                    match next {
                        Link::Nill => {
                            self.data[index] = Option::None;
                        }
                        Link::Next(n) => {
                            self.data[index] = Some(n);
                        }
                    }
                } else {
                    // remove the node in the middle
                    let mut next = &mut x.next;
                    loop {
                        match next {
                            Link::Nill => break,
                            Link::Next(n) if n.key == key => {
                                let nnext = mem::replace(&mut n.next, Link::Nill);
                                *next = nnext;
                                break;
                            }
                            Link::Next(n) => {
                                next = &mut n.next;
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_init() {
        let h: HashTableLinked<i32> = HashTableLinked::new();
        assert_eq!(h.get(0), None);
    }

    #[test]
    fn set_get() {
        let mut h = HashTableLinked::new();
        h.set(0, 0);
        assert_eq!(h.get(0), Some(&0));
    }

    #[test]
    fn set_get_update() {
        let mut h = HashTableLinked::new();
        h.set(0, 0);
        assert_eq!(h.get(0), Some(&0));

        h.set(0, 1);
        assert_eq!(h.get(0), Some(&1));
    }

    #[test]
    fn set_delete() {
        let mut h = HashTableLinked::new();
        h.set(0, 0);
        h.set(1, 1);
        h.delete(1);

        assert_eq!(h.get(0), Some(&0));
        assert_eq!(h.get(1), None);
    }

    #[test]
    fn set_delete_get_collision() {
        let mut h = HashTableLinked::new();

        for i in 0..2000 {
            h.set(i, i);
        }
        for i in 0..2000 {
            assert_eq!(h.get(i), Some(&i));
        }

        for i in 0..1000 {
            h.delete(i);
        }
        for i in 0..2000 {
            if i < 1000 {
                assert_eq!(h.get(i), None);    
            } else {
                assert_eq!(h.get(i), Some(&i));
            }
        }
    }
}
