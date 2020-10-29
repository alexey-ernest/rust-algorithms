/// Hash table linked list implementation (TODO).
pub struct HashTableLinked<T> {
    data: Vec<Option<Node<T>>>,
}

#[derive(Clone)]
struct Node<T> {
    key: usize,
    value: T,
    next: Box<Option<Node<T>>>,
}

const INIT_SIZE: usize = 997;

impl<T> HashTableLinked<T> where T: Copy {
    pub fn new() -> HashTableLinked<T> {
        HashTableLinked {
            data: vec![Option::None; INIT_SIZE],
        }
    }

    pub fn set(&mut self, key: usize, value: T) {
        let index = key % self.data.len();
        let mut x = &mut self.data[index];
        while let Some(ref mut node) = x {
            if node.key == key {
                // found, updating value
                node.value = value;
                return;
            }
            x = &mut (*node.next);
        }

        // not found
        let next = self.data[index].take();
        self.data[index] = Option::Some(Node { 
            key: key, 
            value: value, 
            next: Box::new(next) 
        });
    }

    pub fn get(&self, key: usize) -> Option<T> {
        let index = key % self.data.len();
        let mut x = &self.data[index];
        while let Some(ref node) = x {
            if node.key == key {
                return Option::Some(node.value);
            }
            x = &(*node.next);
        }

        Option::None
    }

    // pub fn delete(&mut self, key: usize) {
    //     let index = key % self.data.len();
    //     let mut x = &mut self.data[index];
    //     if let Some(ref mut node) = x {
    //         if node.key == key {
    //             let next = &node.next;
    //             node.next = Box::new(Option::None);
    //             self.data[index] = next;
    //             return;
    //         }
    //     }
        
    //     // while let Some(ref mut node) = x {
    //     //     // if node.key == key {
    //     //     //     if prev.is_some() {
    //     //     //         prev.
    //     //     //     }
    //     //     // }
    //     //     x = &mut (*node.next);
    //     // }

    //     panic!("the key {:?} does not exist", key);
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init_ht() {
        let h: HashTableLinked<i32> = HashTableLinked::new();
        assert_eq!(h.get(0), Option::None);
    }

    #[test]
    fn get_set() {
        let mut h = HashTableLinked::new();
        h.set(0, 0);
        h.set(1, 1);

        assert_eq!(h.get(0), Option::Some(0));
        assert_eq!(h.get(1), Option::Some(1));
        assert_eq!(h.get(2), Option::None);
    }
}